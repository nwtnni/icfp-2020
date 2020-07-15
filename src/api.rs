use std::env;
use std::fmt;

use anyhow::anyhow;
use anyhow::Context as _;
use reqwest::blocking;

/// Responsible for communicating with the central ICFP server.
///
/// Abstracts over the communication protocol and transport method.
#[allow(unused)]
pub struct Client {
    inner: blocking::Client,
    key: String,
    url: String,
}

impl fmt::Debug for Client {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("icfp::api::Client")
            .field("url", &self.url)
            .finish()
    }
}

impl Client {
    pub fn new() -> anyhow::Result<Self> {
        let mut args = env::args().skip(1);

        let url = args
            .next()
            .or_else(|| env::var("ICFP_SERVER_URL").ok())
            .ok_or_else(Self::usage)?;

        let key = args
            .next()
            .or_else(|| env::var("ICFP_API_KEY").ok())
            .ok_or_else(Self::usage)?;

        let client = blocking::Client::new();

        let response = client
            .post(&url)
            .body(key.clone())
            .send()
            .and_then(blocking::Response::error_for_status)
            .and_then(blocking::Response::text)
            .with_context(|| anyhow!("Failed to register against server"))?;

        log::debug!("Registered: '{}'", response);

        Ok(Client { inner: client, key, url })
    }

    fn usage() -> anyhow::Error {
        anyhow!("Usage: icfp <SERVER_URL> <API_KEY>")
    }
}
