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
    pub state: State,
}

impl From<&Exp> for Option<Response> {
    fn from(exp: &Exp) -> Self {
        let (status, tail) = exp.to_cons();
        let status = status.to_int();

        match status {
        | 1 => {
            let (stage, tail) = tail.to_cons();
            let stage = Stage::from(&**stage);

            let (info, tail) = tail.to_cons();
            let info = Info::from(&**info);

            let (state, _) = tail.to_cons();
            let state = State::from(&**state);

            Some(Response { info, stage, state })
        }
        | _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Info {
    pub role: Role,
}

impl From<&Exp> for Info {
    fn from(exp: &Exp) -> Self {
        let (_x0, tail) = exp.to_cons();
        let (role, _) = tail.to_cons();
        let role = Role::from(&**role);
        Info { role }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    /// Time inside the game.
    tick: i64,

    /// All ships and previously applied commands.
    ships: Vec<(Ship, Vec<Command>)>,
}

impl From<&Exp> for State {
    fn from(exp: &Exp) -> Self {
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
                let command = Command::from(&**command_exp);
                commands.push(command);
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
    }
}

impl From<&Exp> for Command {
    fn from(exp: &Exp) -> Self {
        let (r#type, tail) = exp.to_cons();
        let r#type = r#type.to_int();

        let (id, tail) = tail.to_cons();
        let id = id.to_int();

        match r#type {
        | 0 => {
            let (vec, _) = tail.to_cons();
            let (x, y) = vec.to_cons();
            let x = x.to_int();
            let y = y.to_int();
            Command::Accelerate { id, x, y }
        }
        | 1 =>  Command::Detonate { id },
        | 2 => {
            let (target, _) = tail.to_cons();
            let (x, y) = target.to_cons();
            let x = x.to_int();
            let y = y.to_int();
            Command::Shoot { id, x, y }
        }
        | other => panic!(format!("Expected 0, 1, or 2 for command type, but found: {}", other)),
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
                Exp::Atom(Atom::Var(3)),
            )
        }
        };
        Rc::try_unwrap(list)
            .expect("Impossible: command Rc has single owner")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ship {
    role: Role,
    id: i64,
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl From<&Exp> for Ship {
    fn from(exp: &Exp) -> Self {
        let (role, tail) = exp.to_cons();
        let role = Role::from(&**role);

        let (id, tail) = tail.to_cons();
        let id = id.to_int();

        let (pos, tail) = tail.to_cons();
        let (x, y) = pos.to_cons();
        let x = x.to_int();
        let y = y.to_int();

        // Discard remaining x4, x5, x6, x7 list elements
        let (vel, _) = tail.to_cons();
        let (vx, vy) = vel.to_cons();
        let vx = vx.to_int();
        let vy = vy.to_int();

        Ship {
            role,
            id,
            x,
            y,
            vx,
            vy,
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
