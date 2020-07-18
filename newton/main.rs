use std::env;
use std::fs;
use std::rc::Rc;

use anyhow::anyhow;

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

    match mode {
        Mode::Protocol => {
            let protocol = Rc::new(icfp::parse::interaction_protocol(tokens));
            let temp = &protocol[1043];
            dbg!(icfp::eval(temp, &protocol));
        }
        Mode::Test => {
            let test = icfp::parse::test_suite(tokens);
            let protocol = Rc::new(icfp::ast::Protocol::default());
            dbg!(&test);
            for t in test.equals {
                let lhs = icfp::eval(&t.lhs, &protocol);
                let rhs = icfp::eval(&t.rhs, &protocol);
                assert_eq!(lhs, rhs)
            }
        }
    }

    Ok(())
}
