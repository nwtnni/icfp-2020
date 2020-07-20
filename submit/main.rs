use std::env;

use rand::Rng as _;

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

    client.join(&mut atoms)?;

    let stats = game::Stats {
        fuel: 256,
        damage: 0,
        coolant: 8,
        spawns: 32,
    };

    let mut current = client.start(&mut atoms, &stats)?;
    let mut commands = Vec::new();
    let mut rng = rand::thread_rng();

    let team = current.info.role;

    while current.stage != game::Stage::Finished {

        let state = current
            .state
            .expect("Missing game state");

        log::info!("Tick {}", state.tick);

        commands.clear();

        for (ally, _) in state.ships.iter().filter(|(ship, _)| ship.role == team) {

            // Skip dummy ships with no fuel
            if ally.stats.fuel == 0 {
                continue;
            }

            let speed = ally.vx.pow(2) + ally.vy.pow(2);

            if ally.stats.fuel > 64 && speed > 100 {
                commands.push(game::Command::Split {
                    id: ally.id,
                    stats: game::Stats {
                        fuel: 6,
                        damage: 0,
                        coolant: 0,
                        spawns: 1,
                    },
                })
            } else if ally.temp <= ally.max_temp / 2 {
                let (dx, dy) = direction(ally);

                let sign = match speed {
                | 000..=049 if rng.gen_ratio(1, 8) => 2,
                | 000..=049 => 1,
                | 050..=100 => continue,
                | _ => -1,
                };

                commands.push(game::Command::Accelerate {
                    id: ally.id,
                    x: dx * sign,
                    y: dy * sign,
                })
            }
        }

        current = client.commands(&mut atoms, &commands)?;
    }

    Ok(())
}

fn direction(ship: &game::Ship) -> (i64, i64) {
    match (ship.x >= 0, ship.y >= 0) {
    | (true, true) => (-1, 1),
    | (false, true) => (-1, -1),
    | (false, false) => (1, -1),
    | (true, false) => (1, 1),
    }
}
