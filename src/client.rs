use crate::{error::EzCurlError, request::HttpRequest, response::HttpResponse};

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn send(&self, http_request: &HttpRequest) -> Result<HttpResponse, EzCurlError> {
        let mut builder = self.client.request(
            http_request.method().as_reqwest_method(),
            http_request.url(),
        );

        for (name, value) in http_request.header_values()? {
            builder = builder.header(name, value);
        }

        if let Some(body) = http_request.body() {
            builder = builder.body(body.to_vec());
        }

        let response = builder.send().await?;

        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    value.to_str().unwrap_or("<invalid header>").to_string(),
                )
            })
            .collect();

        let body = response.bytes().await?.to_vec();
        Ok(HttpResponse::new(status, headers, body))
    }
}
