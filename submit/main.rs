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

    let stats = match initial.info.role {
    | game::Role::Attack => game::Stats { fuel: 128, damage: 64, coolant: 8, spawns: 1, },
    | game::Role::Defend => game::Stats { fuel: 128, damage: 0, coolant: 16, spawns: 96, },
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

        match team {
        | game::Role::Defend => {
            for (ally, _) in state.ships.iter().filter(|(ship, _)| ship.role == team) {

                // Skip dummy ships with no fuel
                if ally.stats.fuel == 0 {
                    continue;
                }

                let speed = ally.vx.pow(2) + ally.vy.pow(2);

                if ally.stats.fuel > 64 && speed > 64 {
                    commands.push(game::Command::Split(game::Stats {
                        fuel: 0,
                        damage: 0,
                        coolant: 0,
                        spawns: 1,
                    }))
                } else if ally.temp <= ally.max_temp / 2 {
                    let (dx, dy) = direction(ally);

                    let sign = match speed {
                    | 000..=064 => 1,
                    | 065..=128 => 0,
                    | _ => -1,
                    };

                    commands.push(game::Command::Accelerate {
                        id: ally.id,
                        x: tweak(dx * sign, &mut rng),
                        y: tweak(dy * sign, &mut rng),
                    })
                }
            }
        }
        | game::Role::Attack => {
            for (ally, _) in state.ships.iter().filter(|(ship, _)| ship.role == team) {

                let mut min_dist = i64::MAX;
                let mut min_ship = None;

                for (enemy, _) in state.ships.iter().filter(|(ship, _)| ship.role != team) {
                    let dist = ((ally.x + ally.vx) - (enemy.x + enemy.vx)).pow(2) +
                        ((ally.y + ally.vy) - (enemy.y + enemy.vy)).pow(2);

                    if dist < min_dist {
                        min_dist = dist;
                        min_ship = Some(*enemy);
                    }
                }

                if let Some(enemy) = min_ship {
                    if ally.temp <= ally.max_temp / 2 {

                        let (dx, dy) = direction(&ally);
                        let ally_speed = ally.vx.pow(2) + ally.vy.pow(2);
                        let enemy_speed = enemy.vx.pow(2) + enemy.vy.pow(2);

                        let sign = match ally_speed {
                        | i64::MIN..=048 => 1,
                        | 049..=127 if ally_speed < enemy_speed.saturating_sub(16) => 1,
                        | 049..=127 if ally_speed > enemy_speed.saturating_add(16) => -1,
                        | 049..=127 => 0,
                        | 128..=i64::MAX => -1,
                        };

                        commands.push(game::Command::Accelerate {
                            id: ally.id,
                            x: dx * sign,
                            y: dy * sign,
                        });
                    }

                    if ally.temp <= (ally.max_temp - ally.stats.damage) {
                        commands.push(game::Command::Shoot {
                            id: enemy.id,
                            x: enemy.x + enemy.vx,
                            y: enemy.y + enemy.vy,
                        });
                    }
                }
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

fn tweak<R: rand::Rng>(x: i64, rng: &mut R) -> i64 {
    if rng.gen_ratio(1, 8) {
        match (x, rng.gen()) {
        | (0, true) => 1,
        | (0, false) => -1,
        | (1, true) => 0,
        | (1, false) => -1,
        | (_, true) => 1,
        | (_, false) => 0,
        }
    } else {
        x
    }
}
