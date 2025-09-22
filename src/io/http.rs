use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_streams::ReadableStream;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response, window};

#[derive(Debug)]
pub enum ClientError {
    Js(String),
    Http(u16),
    Response(String),
}

impl From<JsValue> for ClientError {
    fn from(err: JsValue) -> Self {
        ClientError::Js(format!("{:?}", err))
    }
}

pub struct HttpClient;

impl HttpClient {
    /// Helper to perform a fetch and return a valid Response or error
    async fn fetch_response(request: &Request) -> Result<Response, ClientError> {
        let window = window().ok_or_else(|| ClientError::Js("No window object".to_string()))?;
        let response_value = JsFuture::from(window.fetch_with_request(request)).await?;
        let response: Response = response_value.dyn_into().map_err(|e| {
            ClientError::Js(format!(
                "Failed to convert fetch result to Response: {:?}",
                e
            ))
        })?;
        if !response.ok() {
            return Err(ClientError::Http(response.status()));
        }
        Ok(response)
    }

    /// Get the file size in bytes by making a HEAD request
    pub async fn get_file_size(url: &str) -> Result<u64, ClientError> {
        let opts = RequestInit::new();
        opts.set_method("HEAD");
        opts.set_mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)?;
        let response = Self::fetch_response(&request).await?;

        // Get file size from Content-Length header - required
        let no_content_length_error =
            || ClientError::Response("no valid content-length header found".to_string());
        match response.headers().get("content-length") {
            Ok(Some(content_length_str)) => content_length_str
                .trim()
                .parse::<u64>()
                .map_err(|_| no_content_length_error()),
            _ => Err(no_content_length_error()),
        }
    }

    /// Fetch a range of bytes by making a GET request with Range header
    pub async fn fetch_range(
        url: &str,
        start: u64,
        end: u64,
    ) -> Result<impl futures::io::AsyncRead + Unpin, ClientError> {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let headers = Headers::new()?;
        headers.set("Range", &format!("bytes={}-{}", start, end))?;
        opts.set_headers(&headers);

        let request = Request::new_with_str_and_init(url, &opts)?;
        let response = Self::fetch_response(&request).await?;

        // Get the response body as a JS ReadableStream
        let body = response
            .body()
            .ok_or_else(|| ClientError::Response("Response has no body".to_string()))?;

        ReadableStream::from_raw(body)
            .try_into_async_read()
            .map_err(|e| {
                ClientError::Js(format!(
                    "Failed to convert body to futures::io::AsyncRead: {e:?}"
                ))
            })
    }
}
