use anyhow::Result;
use http::{request, Method, Request};
use wasi_experimental_http::Response;

pub struct Http {}

impl Http {
    pub fn get(url: &str) -> Result<Response> {
        let req = request::Builder::new()
            .method(Method::GET)
            .uri(url)
            .body(None)?;
        let resp = wasi_experimental_http::request(req)?;
        Ok(resp)
    }
}
