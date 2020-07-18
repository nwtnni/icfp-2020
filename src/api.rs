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

        Ok(Client { inner: client, key, url })
    }

    pub fn get_alien_response(&self, id: &str) -> anyhow::Result<String> {
        log::info!("Retrieving alien response for id '{}'", id);
        self.inner
            .get(&format!("{}/aliens/{}", &self.url, id))
            .query(&[("apiKey", &self.key)])
            .send()
            .and_then(Self::extract)
            .with_context(|| anyhow!("Failed to retrieve alien response for id '{}'", id))
    }

    pub fn send_alien_message(&self, message: String) -> anyhow::Result<String> {
        log::info!("Sending alien message");
        self.inner
            .post(&format!("{}/aliens/send", &self.url))
            .query(&[("apiKey", &self.key)])
            .body(message)
            .send()
            .and_then(Self::extract)
            .with_context(|| anyhow!("Failed to send alien message"))
    }

    fn extract(response: blocking::Response) -> reqwest::Result<String> {
        log::info!("Received response: {:#?}", &response);
        response
            .error_for_status()
            .and_then(blocking::Response::text)
    }

    fn usage() -> anyhow::Error {
        anyhow!("Usage: icfp <SERVER_URL> <API_KEY>")
    }
}
