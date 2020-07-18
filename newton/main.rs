use std::env;
use std::fs;

use anyhow::anyhow;
use typed_arena::Arena;

#[derive(Copy, Clone, Debug)]
enum Mode {
    Test,
    Protocol,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut args = env::args().skip(1);

    let mode = match args.next().as_deref() {
        Some("t") | Some("test") => Mode::Test,
        Some("p") | Some("protocol") => Mode::Protocol,
        other => {
            return Err(anyhow!(
                "Unknown mode '{:?}', expected '[t]est' or '[p]rotocol'",
                other
            ))
        }
    };

    let path = args.next().unwrap();
    let transmission = fs::read_to_string(&path)?;
    let tokens = icfp::lex(&transmission);
    let arena = Arena::new();

    match mode {
        Mode::Protocol => {
            let protocol = icfp::parse::interaction_protocol(&arena, tokens);
            dbg!(protocol);
        }
        Mode::Test => {
            let test = icfp::parse::test_suite(&arena, tokens);
            dbg!(&test);
            for t in test.equals {
                let lhs = icfp::eval(&arena, &t.lhs);
                let rhs = icfp::eval(&arena, &t.rhs);
                assert_eq!(lhs, rhs)
            }
        }
    }

    Ok(())
}
