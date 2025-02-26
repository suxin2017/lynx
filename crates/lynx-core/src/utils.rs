use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Error, anyhow};
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Bytes;

pub fn host_addr(uri: &http::Uri) -> Option<String> {
    uri.authority().map(|auth| auth.to_string())
}

pub fn empty() -> BoxBody<Bytes, Error> {
    Empty::<Bytes>::new()
        .map_err(|never| anyhow!(never))
        .boxed()
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Error> {
    Full::new(chunk.into())
        .map_err(|never| anyhow!(never))
        .boxed()
}

pub fn is_http(uri: &http::Uri) -> bool {
    uri.scheme_str().map(|s| s == "http").unwrap_or(false)
}

pub fn is_https(uri: &http::Uri) -> bool {
    matches!(uri.port_u16(), Some(443))
}

pub fn get_current_timestamp_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

pub async fn read_file(file_path: &PathBuf) -> anyhow::Result<Vec<u8>> {
    let content = tokio::fs::read(file_path).await?;
    Ok(content)
}
