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
    | game::Role::Attack => game::Stats { fuel: 124, damage: 64, coolant: 8, bombs: 1, },
    | game::Role::Defend => game::Stats { fuel: 252, damage: 0, coolant: 16, bombs: 1, },
    };

    let mut current = client.start(&mut atoms, &stats)?;
    let mut commands = Vec::new();

    let team = current.info.role;

    while current.stage != game::Stage::Finished {

        let state = current
            .state
            .expect("Missing game state");

        log::info!("Tick {}", state.tick);

        commands.clear();

        commands.extend(
            state
                .ships
                .iter()
                .filter(|(ship, _)| ship.role == team)
                .flat_map(|(ship, _)| {
                    if ship.temp > ship.max_temp / 2 {
                        return None;
                    }

                    let (dx, dy) = match (ship.x >= 0, ship.y >= 0) {
                    | (true, true) => (-1, 1),
                    | (false, true) => (-1, -1),
                    | (false, false) => (1, -1),
                    | (true, false) => (1, 1),
                    };

                    let sign = match ship.vx.pow(2) + ship.vy.pow(2) {
                    | 000..=064 => 1,
                    | 065..=128 => 0,
                    | _ => -1,
                    };

                    Some(game::Command::Accelerate {
                        id: ship.id,
                        x: dx * sign,
                        y: dy * sign,
                    })
                })
        );

        if team == game::Role::Attack {
            for (ally, _) in state.ships.iter().filter(|(ship, _)| ship.role == team) {

                if ally.temp > (ally.max_temp / 2 - ally.stats.damage) {
                    continue;
                }

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
                    commands.push(game::Command::Shoot {
                        id: enemy.id,
                        x: enemy.x + enemy.vx,
                        y: enemy.y + enemy.vy,
                    });
                }
            }
        }

        current = client.commands(&mut atoms, &commands)?;
    }

    Ok(())
}
