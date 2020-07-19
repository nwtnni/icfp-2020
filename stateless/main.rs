use std::fs;
use std::rc::Rc;
use std::time;
use std::thread;
use std::io;

use icfp::ast::Atom;
use icfp::ast::AtomCache;
use icfp::ast::Exp;

fn main() -> anyhow::Result<()> {

    env_logger::init();

    let client = icfp::Client::new()?;

    let file = fs::read_to_string("data/galaxy.txt")?;
    let tokens = icfp::lex(&file);
    let protocol = icfp::parse::interaction_protocol(tokens);

    let mut cache = AtomCache::default();
    let nil = cache.get(Atom::Nil);
    let mut state = Rc::clone(&nil);

    let mut input = String::new();
    let mut stdin = io::BufReader::new(io::stdin());

    loop {
        let in_state = std::mem::replace(&mut state, Rc::clone(&nil));
        let in_vector = read(&mut input, &mut stdin);

        let (out_state, out_data) = icfp::interact(
            &client,
            &protocol,
            &mut cache,
            in_state,
            in_vector,
        );

        icfp::draw::multidraw(&out_data);
        let _ = std::mem::replace(&mut state, out_state);

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn read<R: io::BufRead>(buffer: &mut String, mut stdin: R) -> Rc<Exp> {
    buffer.clear();
    stdin.read_line(buffer).unwrap();

    let mut iter = buffer.trim().split(',');

    let x = iter
        .next()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    let y = iter
        .next()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    Exp::cons(
        Exp::Atom(Atom::Int(x)),
        Exp::Atom(Atom::Int(y)),
    )
}
