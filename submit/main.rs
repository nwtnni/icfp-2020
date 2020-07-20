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
        fuel: 128,
        damage: 64,
        coolant: 4,
        spawns: 4,
    };

    let mut current = client.start(&mut atoms, &stats)?;
    let mut commands = Vec::new();
    let mut spawned = 0;
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

            if ally.stats.fuel > 64
            && ally.stats.spawns > 1
            && speed > 100
            && state.tick - spawned > 16 {
                commands.push(game::Command::Split {
                    id: ally.id,
                    stats: game::Stats {
                        fuel: 8,
                        damage: 0,
                        coolant: 0,
                        spawns: 1,
                    },
                });
                spawned = state.tick;
                continue;
            }

            if ally.temp <= ally.max_temp / 2 {
                let (dx, dy) = direction(ally);

                let sign = match speed {
                | 000..=064 if rng.gen_ratio(1, 8) => 2,
                | 000..=064 => 1,
                | 065..=144 => continue,
                | _ => -1,
                };

                commands.push(game::Command::Accelerate {
                    id: ally.id,
                    x: dx * sign,
                    y: dy * sign,
                })
            }

            if ally.temp > ally.max_temp / 2
            || ally.stats.damage == 0 {
                continue;
            }

            let mut min_dist = i64::MAX;
            let mut min_ship = None;

            for (enemy, _) in state.ships.iter().filter(|(ship, _)| ship.role != team) {
                let dist = ((ally.x + ally.vx) - (enemy.x + enemy.vx)).pow(2) +
                    ((ally.y + ally.vy) - (enemy.y + enemy.vy)).pow(2);

                if dist < min_dist {
                    min_dist = dist;
                    min_ship = Some(enemy);
                }
            }

            // Same quadrant
            if let Some(enemy) = min_ship {
                if (enemy.x + enemy.vx >= 0) == (ally.x + ally.vx >= 0)
                && (enemy.y + enemy.vy >= 0) == (ally.y + ally.vy >= 0) {
                    commands.push(game::Command::Shoot {
                        id: ally.id,
                        x: enemy.x + enemy.vx,
                        y: enemy.y + enemy.vy,
                        power: ally.stats.damage,
                    });
                }
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
