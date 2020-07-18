use std::env;
use std::fs;

use typed_arena::Arena;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let path = env::args().nth(1).unwrap();
    let transmission = fs::read_to_string(&path)?;

    let tokens = icfp::lex(&transmission);

    let arena = Arena::new();
    let ast = icfp::parse(&arena, tokens);

    dbg!(ast);

    Ok(())
}
