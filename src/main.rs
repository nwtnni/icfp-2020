use std::env;

use anyhow::anyhow;
use anyhow::Context as _;
use reqwest::blocking;

fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);
    let url = args
        .next()
        .or_else(|| env::var("ICFP_SERVER_URL").ok())
        .ok_or_else(usage)?;
    let key = args
        .next()
        .or_else(|| env::var("ICFP_API_KEY").ok())
        .ok_or_else(usage)?;

    let client = blocking::Client::new();

    let response = client
        .post(&url)
        .body(key)
        .send()
        .and_then(blocking::Response::error_for_status)
        .with_context(|| anyhow!("Failed to register against server"))?;

    println!("{}", response.text()?);

    Ok(())
}

fn usage() -> anyhow::Error {
    anyhow!("Usage: icfp <SERVER_URL> <API_KEY>")
}
