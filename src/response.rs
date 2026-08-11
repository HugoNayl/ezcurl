use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }
}
