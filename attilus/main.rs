// use icfp::eval;
// use icfp::lex;
// use icfp::parse::exp;
// use icfp::transport::modulate;
// use icfp::transport::demodulate;
// use icfp::transport::demodulate_list;
// use std::vec::Vec;

// #[allow(dead_code)]
// fn build_vec(vec: &mut Vec<i64>, acc: Value) -> Value {
//     if vec.is_empty() {
//         return acc
//     };
//     // let (x, y) = vec.pop().expect("Empty vec?");
//     let x = vec.pop().expect("Empty vec?");
//     build_vec(
//         vec,
//         Value::Cons(
//             Box::new(
//                 Value::Int(x)
//             ),
//             Box::new(acc)
//         )
//     )
// }

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // let _ = icfp::Client::new()?;
    // let mut draw_vec = Vec::new();
    // draw_vec.push(1);
    // draw_vec.push(2);
    // let draw_args = build_vec(&mut draw_vec, Value::Nil);
    // let draw_args = eval(
    //     dbg!(&exp(
    //         &mut lex("ap ap cons 1 ap ap cons ap ap cons 2 ap ap cons 3 nil ap ap cons 4 nil ")
    //         ).expect("bruh")));
    // print!("{}", modulate_list(draw_args));

//     println!("{:?}", demodulate_list("110110000111011111100001001111110101000000"));
//     println!("{:?}", demodulate_list("110110000111011111100001001111110100110000"));
//     println!("");
//     println!("{:?}", demodulate("1101100001110111110101000110010111000"));

    Ok(())
}
