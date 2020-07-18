use icfp::Value;
use std::vec::Vec;

fn build_vec(vec: &mut Vec<(i64, i64)>, acc: Value) -> Value {
    if vec.is_empty() {
        return acc
    };
    let (x, y) = vec.pop().expect("Empty vec?");
    build_vec(
        vec,
        Value::Cons(
            Box::new(
                Value::Cons(
                    Box::new(Value::Int(x)),
                    Box::new(Value::Int(y)),
                )
            ),
            Box::new(acc)
        )
    )
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // let _ = icfp::Client::new()?;
    let mut draw_vec: Vec<(i64, i64)> = Vec::new();
    draw_vec.push((3, 2));
    draw_vec.push((3, 3));
    draw_vec.push((7, 1));
    draw_vec.push((7, 3));
    let draw_args = build_vec(&mut draw_vec, Value::Nil);

    icfp::draw(&draw_args);

    Ok(())
}
