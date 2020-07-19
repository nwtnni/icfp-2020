// use std::env;
// use std::fs;

// use anyhow::anyhow;

// #[derive(Copy, Clone, Debug)]
// enum Mode {
//     Test,
//     Protocol,
// }

fn build_vec(mut vec: Vec<(i64, i64)>, acc: icfp::Value) -> icfp::Value {
    if vec.is_empty() {
        return acc
    };
    let (x, y) = vec.pop().expect("Empty vec?");
    build_vec(
        vec,
        icfp::Value::Cons(
            Box::new(
                icfp::Value::Cons(
                    Box::new(icfp::Value::Int(x)),
                    Box::new(icfp::Value::Int(y)),
                )
            ),
            Box::new(acc)
        )
    )
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = icfp::Client::new()?;
    let temp = build_vec(vec![(5, 5)], icfp::Value::Nil);


    dbg!(temp);

    // dbg!(icfp::interact(
    //     &client,
    //     icfp::Value::Nil,
    //     temp.clone(),
    // ));

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
