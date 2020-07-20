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

    log::info!("Player Key: {}", player_key);

    let mut atoms = icfp::ast::AtomCache::default();
    let client = icfp::Client::new(
        server_url,
        API_KEY.trim().to_owned(),
        Some(player_key),
    );

    let initial = client.join(&mut atoms)?;

    log::info!("Initial State: {:#?}", initial);

    let stats = game::Stats {
        fuel: 238,
        damage: 0,
        coolant: 32,
        bombs: 1,
    };

    let mut current = client.start(&mut atoms, &stats)?;

    let team = current.info.role;

    while current.stage != game::Stage::Finished {

        let state = current
            .state
            .expect("Missing game state");

        log::info!("Tick {}", state.tick);

        let commands = state
            .ships
            .iter()
            .filter(|(ship, _)| ship.role == team)
            .map(|(ship, _)| game::Command::Accelerate {
                id: ship.id,
                x: ship.vx,
                y: ship.vy,
            })
            .collect::<Vec<_>>();

        current = client.commands(&mut atoms, &commands)?;
    }

    Ok(())
}
