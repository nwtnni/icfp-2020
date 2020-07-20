use std::env;

use icfp::game;

// TODO: wipe from Git history before publicizing repo
static API_KEY: &str = include_str!("../key.txt");

fn main() -> anyhow::Result<()> {

    env_logger::init();

    let mut args = env::args().skip(1);
    let server_url = args.next().unwrap();
    let player_key = args
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let mut atoms = icfp::ast::AtomCache::default();
    let client = icfp::Client::new(
        server_url,
        API_KEY.trim().to_owned(),
        Some(player_key),
    );

    client.join(&mut atoms)?;

    let mut current = client.start(&mut atoms, 1, 2, 3, 4)?;
    let team = current.info.role;

    while current.stage != game::Stage::Finished {

        log::info!("Tick {}", current.state.tick);

        let commands = current
            .state
            .ships
            .iter()
            .filter(|(ship, _)| ship.role == team)
            .map(|(ship, _)| game::Command::Accelerate {
                id: ship.id,
                x: 1,
                y: 1
            })
            .collect::<Vec<_>>();

        current = client.commands(&mut atoms, &commands)?;
    }

    Ok(())
}
