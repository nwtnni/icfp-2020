use std::env;
use std::fmt;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::Context as _;
use reqwest::blocking;

use crate::ast::Atom;
use crate::ast::AtomCache;
use crate::ast::Exp;
use crate::game;
use crate::transport;

/// Responsible for communicating with the central ICFP server.
///
/// Abstracts over the communication protocol and transport method.
#[allow(unused)]
pub struct Client {
    inner: blocking::Client,
    api_key: String,
    url: String,
}

#[derive(Clone, Debug)]
pub struct CreateResponse {
    pub attack: i64,
    pub defend: i64,
}

impl From<&Exp> for CreateResponse {
    fn from(exp: &Exp) -> Self {
        let (_, tail) = exp.to_cons();
        let (keys, _) = tail.to_cons();

        let (attack_list, tail) = keys.to_cons();
        let (defend_list, _) = tail.to_cons();

        // (0, attack_player_key)
        let (_index, tail) = attack_list.to_cons();
        let (attack, _) = tail.to_cons();
        let attack = attack.to_int();

        // (0, defend_player_key)
        let (_index, tail) = defend_list.to_cons();
        let (defend, _) = tail.to_cons();
        let defend = defend.to_int();

        CreateResponse { attack, defend }
    }
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

        let api_key = args
            .next()
            .or_else(|| env::var("ICFP_API_KEY").ok())
            .ok_or_else(Self::usage)?;

        let client = blocking::Client::new();

        Ok(Client { inner: client, api_key, url })
    }

    pub fn get_alien_response(&self, id: &str) -> anyhow::Result<String> {
        log::info!("Retrieving alien response for id '{}'", id);
        self.inner
            .get(&format!("{}/aliens/{}", &self.url, id))
            .query(&[("apiKey", &self.api_key)])
            .send()
            .and_then(Self::extract)
            .with_context(|| anyhow!("Failed to retrieve alien response for id '{}'", id))
    }

    pub fn send_alien_message(
        &self,
        cache: &mut AtomCache,
        message: &Exp,
    ) -> anyhow::Result<Rc<Exp>> {
        log::info!("Sending alien message");
        self.inner
            .post(&format!("{}/aliens/send", &self.url))
            .query(&[("apiKey", &self.api_key)])
            .body(transport::modulate(message))
            .send()
            .and_then(Self::extract)
            .map(|response| transport::demodulate(&response, cache))
            .with_context(|| anyhow!("Failed to send alien message"))
    }

    pub fn create(
        &self,
        cache: &mut AtomCache,
    ) -> anyhow::Result<CreateResponse> {
        self.send_alien_message(
            cache,
            &list!(Exp::from(1), Exp::from(0)),
        )
        .map(|response| {
            CreateResponse::from(&*response)
        })
    }

    pub fn join(
        &self,
        cache: &mut AtomCache,
        player_key: i64,
    ) -> anyhow::Result<game::Response> {
        self.send_alien_message(
            cache,
            &list!(
                Exp::from(2),
                Exp::from(player_key),
                Exp::Atom(Atom::Nil)
            ),
        )
        .and_then(|response| {
            <Option<game::Response>>::from(&*response)
                .ok_or_else(|| anyhow!("Received error response for `join` from server"))
        })
    }

    pub fn start(
        &self,
        cache: &mut AtomCache,
        player_key: i64,
        x0: i64,
        x1: i64,
        x2: i64,
        x3: i64,
    ) -> anyhow::Result<game::Response> {
        self.send_alien_message(
            cache,
            &list!(
                Exp::from(3),
                Exp::from(player_key),
                list!(Exp::from(x0), Exp::from(x1), Exp::from(x2), Exp::from(x3)),
            ),
        )
        .and_then(|response| {
            <Option<game::Response>>::from(&*response)
                .ok_or_else(|| anyhow!("Received error response for `start` from server"))
        })
    }

    pub fn commands(
        &self,
        cache: &mut AtomCache,
        player_key: i64,
        commands: &[game::Command],
    ) -> anyhow::Result<game::Response> {
        let commands = commands
            .iter()
            .rev()
            .fold(cache.get(Atom::Nil), |acc, command| {
                Exp::cons(Exp::from(*command), acc)
            });

        self.send_alien_message(
            cache,
            &list!(
                Exp::from(4),
                Exp::from(player_key),
                commands,
            ),
        )
        .and_then(|response| {
            <Option<game::Response>>::from(&*response)
                .ok_or_else(|| anyhow!("Received error response for `commands` from server"))
        })
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
