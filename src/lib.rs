use std::fs;

use once_cell::sync;

mod api;
pub mod ast;
mod draw;
mod eval;
mod lex;
pub mod parse;
mod token;
pub mod transport;

pub use api::Client;
pub use draw::draw;
pub use eval::eval;
pub use eval::Value;
pub use lex::lex;
pub use token::Token;

pub static PROTOCOL: sync::Lazy<ast::Protocol> = sync::Lazy::new(|| {
    let text = fs::read_to_string("protocol.txt")
        .expect("Expected file 'protocol.txt'");

    let tokens = lex(&text);

    parse::interaction_protocol(tokens)
});
