use std::{
    env, fs,
    fs::OpenOptions,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    request::{HeaderState, HttpMethod, HttpRequest, RequestField},
    response::HttpResponse,
};

const HISTORY_VERSION: u8 = 1;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    request: HttpRequest,
    response: Option<HttpResponse>,
    error: Option<String>,
}

impl HistoryEntry {
    pub fn new(
        request: HttpRequest,
        response: Option<HttpResponse>,
        error: Option<String>,
    ) -> Self {
        Self {
            request,
            response,
            error,
        }
    }

    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    pub fn response(&self) -> Option<&HttpResponse> {
        self.response.as_ref()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

#[derive(Debug)]
pub struct HistoryStore {
    path: Option<PathBuf>,
}

impl HistoryStore {
    pub fn for_current_user() -> Result<Self, HistoryError> {
        let state_home = env::var_os("XDG_STATE_HOME")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .filter(|path| !path.is_empty())
                    .map(|home| PathBuf::from(home).join(".local/state"))
            })
            .ok_or(HistoryError::MissingHome)?;

        Ok(Self::new(state_home.join("ezcurl/history.json")))
    }

    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: Some(path.into()),
        }
    }

    #[cfg(test)]
    pub fn in_memory() -> Self {
        Self { path: None }
    }

    pub fn load(&self) -> Result<Vec<HistoryEntry>, HistoryError> {
        let Some(path) = &self.path else {
            return Ok(Vec::new());
        };
        if !path.exists() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(path)?;
        let stored: StoredHistory = serde_json::from_reader(BufReader::new(file))?;
        if stored.version != HISTORY_VERSION {
            return Err(HistoryError::UnsupportedVersion(stored.version));
        }

        Ok(stored.entries.into_iter().map(HistoryEntry::from).collect())
    }

    pub fn save(&self, entries: &[HistoryEntry]) -> Result<(), HistoryError> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        let parent = path
            .parent()
            .ok_or_else(|| HistoryError::InvalidPath(path.to_owned()))?;
        fs::create_dir_all(parent)?;

        let temporary_path = temporary_path(path);
        let result = self.write_file(&temporary_path, entries).and_then(|()| {
            fs::rename(&temporary_path, path)?;
            Ok(())
        });

        if result.is_err() {
            let _ = fs::remove_file(&temporary_path);
        }
        result
    }

    fn write_file(&self, path: &Path, entries: &[HistoryEntry]) -> Result<(), HistoryError> {
        let mut options = OpenOptions::new();
        options.create(true).truncate(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }

        let mut file = options.open(path)?;
        let stored = StoredHistory {
            version: HISTORY_VERSION,
            entries: entries.iter().map(StoredEntry::from).collect(),
        };
        serde_json::to_writer_pretty(&mut file, &stored)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
        Ok(())
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut file_name = path.file_name().unwrap_or_default().to_os_string();
    file_name.push(".tmp");
    path.with_file_name(file_name)
}

#[derive(Debug, thiserror::Error)]
pub enum HistoryError {
    #[error("HOME and XDG_STATE_HOME are not defined")]
    MissingHome,
    #[error("invalid history path: {0}")]
    InvalidPath(PathBuf),
    #[error("unsupported history version: {0}")]
    UnsupportedVersion(u8),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize)]
struct StoredHistory {
    version: u8,
    entries: Vec<StoredEntry>,
}

#[derive(Serialize, Deserialize)]
struct StoredEntry {
    request: StoredRequest,
    response: Option<HttpResponse>,
    error: Option<String>,
}

impl From<&HistoryEntry> for StoredEntry {
    fn from(entry: &HistoryEntry) -> Self {
        Self {
            request: StoredRequest::from(entry.request()),
            response: entry.response.clone(),
            error: entry.error.clone(),
        }
    }
}

impl From<StoredEntry> for HistoryEntry {
    fn from(entry: StoredEntry) -> Self {
        Self::new(entry.request.into(), entry.response, entry.error)
    }
}

#[derive(Serialize, Deserialize)]
struct StoredRequest {
    method: HttpMethod,
    url: String,
    headers: Vec<StoredHeader>,
    body: String,
}

impl From<&HttpRequest> for StoredRequest {
    fn from(request: &HttpRequest) -> Self {
        Self {
            method: request.method(),
            url: request.url().to_string(),
            headers: request
                .header_editor()
                .rows()
                .iter()
                .filter(|row| row.state() != HeaderState::Pending)
                .map(|row| StoredHeader {
                    key: row.key().to_string(),
                    value: row.value().to_string(),
                })
                .collect(),
            body: request.editor(RequestField::Body).text().to_string(),
        }
    }
}

impl From<StoredRequest> for HttpRequest {
    fn from(request: StoredRequest) -> Self {
        let mut restored = HttpRequest::new(request.method, request.url);
        for header in request.headers {
            restored.add_header(&header.key, &header.value);
        }
        restored.set_body(request.body.into_bytes());
        restored
    }
}

#[derive(Serialize, Deserialize)]
struct StoredHeader {
    key: String,
    value: String,
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::{HistoryEntry, HistoryStore};
    use crate::{
        request::{HttpMethod, HttpRequest, RequestField},
        response::HttpResponse,
    };

    static NEXT_FILE: AtomicU64 = AtomicU64::new(0);

    fn temporary_history_path() -> std::path::PathBuf {
        let id = NEXT_FILE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("ezcurl-history-{}-{id}.json", std::process::id()))
    }

    #[test]
    fn saves_and_restores_complete_history_entries() {
        let path = temporary_history_path();
        let store = HistoryStore::new(&path);
        let mut request = HttpRequest::new(HttpMethod::Patch, "https://example.test".to_string());
        request.add_header("Authorization", "Bearer secret");
        request.set_body(b"payload".to_vec());
        let response = HttpResponse::new(
            202,
            HashMap::from([("content-type".to_string(), "text/plain".to_string())]),
            b"accepted".to_vec(),
        );

        store
            .save(&[HistoryEntry::new(request, Some(response), None)])
            .unwrap();
        let restored = store.load().unwrap();

        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].request().method(), HttpMethod::Patch);
        assert_eq!(restored[0].request().url(), "https://example.test");
        assert_eq!(
            restored[0].request().editor(RequestField::Body).text(),
            "payload"
        );
        assert_eq!(restored[0].response().map(HttpResponse::status), Some(202));
        assert_eq!(
            restored[0].response().map(HttpResponse::body),
            Some(&b"accepted"[..])
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }

        fs::remove_file(path).unwrap();
    }
}
