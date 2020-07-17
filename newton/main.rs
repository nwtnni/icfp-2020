use std::fs;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let transmission = fs::read_to_string("data/galaxy.txt")?;

    for token in icfp::lex(&transmission) {
        dbg!(token?);
    }

    Ok(())
}
