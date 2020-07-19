use std::time;
use std::thread;

fn main() -> anyhow::Result<()> {

    env_logger::init();

    let client = icfp::Client::new()?;

    let mut state = icfp::Value::Nil;
    let vector = icfp::Value::Cons(
        Box::new(icfp::Value::Int(0)),
        Box::new(icfp::Value::Int(0)),
    );

    loop {
        let in_state = std::mem::replace(&mut state, icfp::Value::Nil);

        let out_state = icfp::interact(&client, in_state, vector.clone());

        let _ = std::mem::replace(&mut state, out_state);

        thread::sleep(time::Duration::from_secs(1));
    }
}
