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
    player_key: Option<i64>,
    server_url: String,
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
            .field("server_url", &self.server_url)
            .finish()
    }
}

impl Client {
    pub fn new(
        server_url: String,
        api_key: String,
        player_key: Option<i64>,
    ) -> Self {
        Client {
            inner: blocking::Client::new(),
            api_key,
            player_key,
            server_url,
        }
    }

    pub fn get_alien_response(&self, id: &str) -> anyhow::Result<String> {
        log::info!("Retrieving alien response for id '{}'", id);
        self.inner
            .get(&format!("{}/aliens/{}", &self.server_url, id))
            .query(&[("apiKey", &self.api_key)])
            .send()
            .and_then(Self::extract_text)
            .with_context(|| anyhow!("Failed to retrieve alien response for id '{}'", id))
    }

    pub fn send_alien_message(
        &self,
        cache: &mut AtomCache,
        message: &Exp,
    ) -> anyhow::Result<Rc<Exp>> {
        log::debug!("Sending alien message: {}", &message);
        self.inner
            .post(&format!("{}/aliens/send", &self.server_url))
            .query(&[("apiKey", &self.api_key)])
            .body(transport::modulate(message))
            .send()
            .and_then(Self::extract_text)
            .map(|response| transport::demodulate(&response, cache))
            .map(|response| {
                log::debug!("Received alien response: {}", &response);
                response
            })
            .with_context(|| anyhow!("Failed to send alien message"))
    }

    pub fn create(
        &self,
        cache: &mut AtomCache,
    ) -> anyhow::Result<CreateResponse> {
        let message = list!(Exp::from(1), Exp::from(0));
        log::debug!("Sending `create` message: {}", &message);
        self.send_alien_message(cache, &message)
            .map(|response| {
                CreateResponse::from(&*response)
            })
    }

    pub fn join(
        &self,
        cache: &mut AtomCache,
    ) -> anyhow::Result<()> {
        let message = list!(
            Exp::from(2),
            Exp::from(self.player_key.expect("Missing player key")),
            Exp::Atom(Atom::Nil)
        );
        log::debug!("Sending `join` message: {}", &message);
        self.send_alien_message(cache, &message)?;
        Ok(())
    }

    pub fn start(
        &self,
        cache: &mut AtomCache,
        x0: i64,
        x1: i64,
        x2: i64,
        x3: i64,
    ) -> anyhow::Result<game::Response> {
        let message = list!(
            Exp::from(3),
            Exp::from(self.player_key.expect("Missing player key")),
            list!(Exp::from(x0), Exp::from(x1), Exp::from(x2), Exp::from(x3)),
        );
        log::debug!("Sending `start` message: {}", &message);
        self.send_alien_message(cache, &message)
            .and_then(Self::extract_game)
    }

    pub fn commands(
        &self,
        cache: &mut AtomCache,
        commands: &[game::Command],
    ) -> anyhow::Result<game::Response> {
        let commands = commands
            .iter()
            .rev()
            .fold(cache.get(Atom::Nil), |acc, command| {
                Exp::cons(Exp::from(*command), acc)
            });

        log::debug!("Sending `commands` message: {}", &commands);

        self.send_alien_message(
            cache,
            &list!(
                Exp::from(4),
                Exp::from(self.player_key.expect("Missing player key")),
                commands,
            ),
        )
        .and_then(Self::extract_game)
    }

    fn extract_text(response: blocking::Response) -> reqwest::Result<String> {
        response
            .error_for_status()
            .and_then(blocking::Response::text)
    }

    fn extract_game(response: Rc<Exp>) -> anyhow::Result<game::Response> {
        <Option<game::Response>>::from(&*response)
            .ok_or_else(|| anyhow!("Received error response from server"))
    }
}
