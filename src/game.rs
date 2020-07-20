use std::rc::Rc;

use crate::ast::Atom;
use crate::ast::Exp;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Response {
    /// Static game state.
    pub info: Info,

    /// Game completion stage.
    pub stage: Stage,

    /// Dynamic game state.
    pub state: Option<State>,
}

impl From<&Exp> for Option<Response> {
    fn from(exp: &Exp) -> Self {

        log::debug!("Parsing response: {}", exp);

        let (status, tail) = exp.to_cons();
        let status = status.to_int();

        match status {
        | 1 => {
            let (stage, tail) = tail.to_cons();
            let stage = Stage::from(&**stage);

            let (info, tail) = tail.to_cons();
            let info = Info::from(&**info);

            let (state, _) = tail.to_cons();
            let state = match &**state {
            | Exp::Atom(Atom::Nil) => None,
            | list => Some(State::from(list)),
            };

            Some(Response { info, stage, state })
        }
        | _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Info {
    /// Maximum number of game ticks.
    pub ticks: i64,

    /// Attacking or defending role.
    pub role: Role,

    /// If attacking, we have access to enemy statistics.
    pub enemy: Option<Stats>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Stats {
    pub fuel: i64,
    pub damage: i64,
    pub coolant: i64,
    pub bombs: i64,
}

impl From<&Exp> for Stats {
    fn from(exp: &Exp) -> Stats {

        log::debug!("Parsing stats: {}", exp);

        let (fuel, tail) = exp.to_cons();
        let fuel = fuel.to_int();

        let (damage, tail) = tail.to_cons();
        let damage = damage.to_int();

        let (coolant, tail) = tail.to_cons();
        let coolant = coolant.to_int();

        let (bombs, _) = tail.to_cons();
        let bombs = bombs.to_int();
        
        Stats { fuel, damage, coolant, bombs }
    }
}

impl From<Stats> for Exp {
    fn from(stats: Stats) -> Exp {
        let stats = list!(
            Exp::from(stats.fuel),
            Exp::from(stats.damage),
            Exp::from(stats.coolant),
            Exp::from(stats.bombs),
        );
        Rc::try_unwrap(stats)
            .expect("Impossible: stats Rc has single owner")
    }
}

impl From<&Exp> for Info {
    fn from(exp: &Exp) -> Self {

        log::debug!("Parsing info: {}", exp);

        let (ticks, tail) = exp.to_cons();
        let ticks = ticks.to_int();

        let (role, tail) = tail.to_cons();
        let role = Role::from(&**role);

        // (512, 1, 64) ?
        let (_, tail) = tail.to_cons();

        // (16, 128) ?
        let (_, tail) = tail.to_cons();

        let (enemy, _) = tail.to_cons();
        let enemy = match &**enemy {
        | Exp::Atom(Atom::Nil) => None,
        | list => Some(Stats::from(list)),
        };

        Info { ticks, role, enemy }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    /// Time inside the game.
    pub tick: i64,

    /// All ships and previously applied commands.
    pub ships: Vec<(Ship, Vec<Command>)>,
}

impl From<&Exp> for State {
    fn from(exp: &Exp) -> Self {

        log::debug!("Parsing state: {}", exp);

        let (tick, tail) = exp.to_cons();
        let tick = tick.to_int();

        let (_x1, tail) = tail.to_cons();

        let (mut ship_exps, _) = tail.to_cons();

        let mut ships = Vec::new();

        while let Some((head, tail)) = ship_exps.to_cons_opt() {
            
            let (ship_exp, rest) = head.to_cons();
            let ship = Ship::from(&**ship_exp);

            let mut commands = Vec::new();
            let (mut command_exps, _) = rest.to_cons();

            while let Some((command_exp, tail)) = command_exps.to_cons_opt() {
                <Option<Command>>::from(&**command_exp)
                    .map(|command| command.with_id(ship.id))
                    .map(|command| commands.push(command));
                command_exps = tail;
            }
            
            ships.push((ship, commands));
            ship_exps = tail;
        }

        State { tick, ships, }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Command {
    Accelerate {
        id: i64,
        x: i64,
        y: i64,
    },
    Detonate {
        id: i64,
    },
    Shoot {
        id: i64,
        x: i64,
        y: i64,
    },
    Split(Stats),
}

impl Command {
    fn with_id(self, id: i64) -> Self {
        match self {
        | Command::Accelerate { id: _, x, y } => Command::Accelerate { id, x, y },
        | Command::Detonate { .. } => Command::Detonate { id },
        | Command::Shoot { id: _, x, y } => Command::Shoot { id, x, y },
        | Command::Split(stats) => Command::Split(stats),
        }
    }
}

impl From<&Exp> for Option<Command> {
    fn from(exp: &Exp) -> Self {

        log::debug!("Parsing command: {}", exp);

        let (r#type, tail) = exp.to_cons();
        let r#type = r#type.to_int();

        // Dummy ID when parsing from a response
        let id = 0;

        match r#type {
        | 0 => {
            let (vec, _) = tail.to_cons();
            let (x, y) = vec.to_cons();
            let x = x.to_int();
            let y = y.to_int();
            Some(Command::Accelerate { id, x, y })
        }
        | 1 =>  Some(Command::Detonate { id }),
        | 2 => {
            let (target, _) = tail.to_cons();
            let (x, y) = target.to_cons();
            let x = x.to_int();
            let y = y.to_int();
            Some(Command::Shoot { id, x, y })
        }
        | 3 => {
            let (stats, _) = tail.to_cons();
            let stats = Stats::from(&**stats);
            Some(Command::Split(stats))
        }
        | _ => None,
        }
    }
}

impl From<Command> for Exp {
    fn from(command: Command) -> Self {
        let list = match command {
        | Command::Accelerate { id, x, y } => {
            list!(
                Exp::from(0),
                Exp::from(id),
                pair!(Exp::from(x), Exp::from(y)),
            )
        }
        | Command::Detonate { id } => {
            list!(
                Exp::from(2),
                Exp::from(id),
            )
        }
        | Command::Shoot { id, x, y } => {
            list!(
                Exp::from(2),
                Exp::from(id),
                pair!(Exp::from(x), Exp::from(y)),
                Exp::Atom(Atom::Nil),
            )
        }
        | Command::Split(stats) => {
            list!(
                Exp::from(3),
                Exp::from(stats),
            )
        }
        };
        Rc::try_unwrap(list)
            .expect("Impossible: command Rc has single owner")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ship {
    pub role: Role,
    pub id: i64,
    pub x: i64,
    pub y: i64,
    pub vx: i64,
    pub vy: i64,
    pub stats: Stats,
    pub temp: i64,
    pub max_temp: i64,
}

impl From<&Exp> for Ship {
    fn from(exp: &Exp) -> Self {

        log::debug!("Parsing ship: {}", exp);

        let (role, tail) = exp.to_cons();
        let role = Role::from(&**role);

        let (id, tail) = tail.to_cons();
        let id = id.to_int();

        let (pos, tail) = tail.to_cons();
        let (x, y) = pos.to_cons();
        let x = x.to_int();
        let y = y.to_int();

        let (vel, tail) = tail.to_cons();
        let (vx, vy) = vel.to_cons();
        let vx = vx.to_int();
        let vy = vy.to_int();

        let (stats, tail) = tail.to_cons();
        let stats = Stats::from(&**stats);

        let (temp, tail) = tail.to_cons();
        let temp = temp.to_int();

        let (max_temp, _) = tail.to_cons();
        let max_temp = max_temp.to_int();

        Ship {
            role,
            id,
            x,
            y,
            vx,
            vy,
            stats,
            temp,
            max_temp,
        }
    }
}

impl From<Ship> for Exp {
    fn from(ship: Ship) -> Exp {
        let ship = list!(
            Exp::from(ship.role),
            Exp::from(ship.id),
            pair!(Exp::from(ship.x), Exp::from(ship.y)),
            pair!(Exp::from(ship.vx), Exp::from(ship.vy)),
            Exp::Atom(Atom::Var(4)),
            Exp::Atom(Atom::Var(5)),
            Exp::Atom(Atom::Var(6)),
            Exp::Atom(Atom::Var(7)),
        );

        Rc::try_unwrap(ship)
            .expect("Impossible: ship Rc has single owner")
    }
}

impl From<i64> for Exp {
    fn from(int: i64) -> Exp {
        Exp::Atom(Atom::Int(int))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Attack,
    Defend,
}

impl From<&Exp> for Role {
    fn from(exp: &Exp) -> Self {

        log::debug!("Parsing role: {}", exp);

        match exp {
        | Exp::Atom(Atom::Int(0)) => Role::Attack,
        | Exp::Atom(Atom::Int(1)) => Role::Defend,
        | _ => panic!(format!("Expected 0 or 1 for role, but found: {}", exp)),
        }
    }
}

impl From<Role> for Exp {
    fn from(role: Role) -> Self {
        match role {
        | Role::Attack => Exp::Atom(Atom::Int(0)),
        | Role::Defend => Exp::Atom(Atom::Int(1)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Stage {
    NotStarted,
    Started,
    Finished,
}

impl From<&Exp> for Stage {
    fn from(exp: &Exp) -> Self {

        log::debug!("Parsing stage: {}", exp);

        match exp {
        | Exp::Atom(Atom::Int(0)) => Stage::NotStarted,
        | Exp::Atom(Atom::Int(1)) => Stage::Started,
        | Exp::Atom(Atom::Int(2)) => Stage::Finished,
        | _ => panic!(format!("Expected 0, 1, or 2 for stage, but found: {}", exp)),
        }
    }
}

impl From<Stage> for Exp {
    fn from(stage: Stage) -> Self {
        match stage {
        | Stage::NotStarted => Exp::Atom(Atom::Int(0)),
        | Stage::Started => Exp::Atom(Atom::Int(1)),
        | Stage::Finished => Exp::Atom(Atom::Int(2)),
        }
    }
}
