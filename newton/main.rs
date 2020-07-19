// use std::env;
// use std::fs;

// use anyhow::anyhow;

// #[derive(Copy, Clone, Debug)]
// enum Mode {
//     Test,
//     Protocol,
// }

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = icfp::Client::new()?;

    let mut tokens = icfp::lex("ap ap cons 0 nil");
    let tree = icfp::parse::exp(&mut tokens).unwrap();
    let list = dbg!(icfp::eval(&tree));
    let modulated = icfp::transport::modulate_list(list);

    dbg!(client.send_alien_message(modulated)?);

    // let mut args = env::args().skip(1);

    // let mode = match args.next().as_deref() {
    //     Some("t") | Some("test") => Mode::Test,
    //     Some("p") | Some("protocol") => Mode::Protocol,
    //     other => {
    //         return Err(anyhow!(
    //             "Unknown mode '{:?}', expected '[t]est' or '[p]rotocol'",
    //             other
    //         ))
    //     }
    // };

    // let path = args.next().unwrap();
    // let transmission = fs::read_to_string(&path)?;
    // let tokens = icfp::lex(&transmission);

    // match mode {
    //     Mode::Protocol => {
    //         let entry = icfp::PROTOCOL.galaxy;
    //         let expr = &icfp::PROTOCOL[entry];
    //         dbg!(expr);
    //     }
    //     Mode::Test => {
    //         let test = icfp::parse::test_suite(tokens);
    //         dbg!(&test);
    //         for t in test.equals {
    //             let lhs = dbg!(icfp::eval(&t.equal.lhs));
    //             let rhs = dbg!(icfp::eval(&t.equal.rhs));
    //             assert_eq!(lhs, rhs)
    //         }
    //     }
    // }

    Ok(())
}
