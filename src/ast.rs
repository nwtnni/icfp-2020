use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops;

/// Interaction protocol.
#[derive(Clone, Debug, Default)]
pub struct Protocol {
    pub assignments: HashMap<u64, Rc<Exp>>,
    pub galaxy: u64,
}

impl ops::Index<u64> for Protocol {
    type Output = Rc<Exp>;
    fn index(&self, var: u64) -> &Self::Output {
        &self.assignments[&var]
    }
}

#[derive(Clone, Debug, Eq)]
pub enum Exp {
    Atom(Atom),
    App(Rc<Exp>, Rc<Exp>, RefCell<Option<Rc<Exp>>>),
}

impl Exp {
    pub fn app<L, R>(lhs: L, rhs: R) -> Rc<Exp>
    where L: Into<Rc<Exp>>,
          R: Into<Rc<Exp>>,
    {
        Rc::new(Exp::App(lhs.into(), rhs.into(), Default::default()))
    }

    pub fn get_cached(&self) -> Option<&Rc<Exp>> {
        match self {
        | Exp::Atom(_) => None,
        | Exp::App(_, _, cache) => cache.borrow().as_ref(),
        }
    }

    pub fn set_cached(&self, exp: Rc<Exp>) {
        match self {
        | Exp::Atom(_) => (),
        | Exp::App(_, _, cache) => *cache.borrow_mut() = Some(exp),
        }
    }

    pub fn to_int(&self) -> i64 {
        match self {
        | Exp::Atom(Atom::Int(int)) => *int,
        | other => panic!(format!("Expected `<INT>`, but found: {:?}", other)),
        }
    }

    pub fn to_cons(&self) -> (&Rc<Exp>, &Rc<Exp>) {
        let (cons_car, cdr) = match self {
        | Exp::Atom(atom) => panic!(format!("Expected `ap ap cons <CAR> <CDR>`, but found: {:?}", atom)),
        | Exp::App(cons_car, cdr, _) => (cons_car, cdr),
        };

        let (cons, car) = match &**cons_car {
        | Exp::Atom(atom) => panic!(format!("Expected `ap cons <CAR>`, but found: {:?}", atom)),
        | Exp::App(cons, car, _) => (cons, car),
        };

        match &**cons {
        | Exp::Atom(Atom::Cons) => (car, cdr),
        | other => panic!(format!("Expected `cons`, but found: {:?}", other)),
        }
    }
}

impl PartialEq for Exp {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
        | (Exp::Atom(lhs), Exp::Atom(rhs)) => lhs == rhs,
        | (Exp::App(llhs, lrhs, _), Exp::App(rlhs, rrhs, _)) => llhs == rlhs && rlhs == rrhs,
        }
    }
}

#[derive(Debug, Default)]
pub struct AtomCache(HashMap<Atom, Rc<Exp>>);

impl AtomCache {
    pub fn get(&mut self, atom: Atom) -> Rc<Exp> {
        // Integers and variables are potentially unbounded,
        // so we avoid caching them to save memory.
        match atom {
        | Atom::Int(_)
        | Atom::Var(_) => Rc::new(Exp::Atom(atom)),
        | _ => {
            self.0
                .entry(atom)
                .or_insert_with(|| Rc::new(Exp::Atom(atom)))
                .clone()
        }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Atom {
    Nil,
    Int(i64),
    Var(u64),
    Bool(bool),

    Neg,
    Inc,
    Dec,

    Add,
    Mul,
    Div,

    Eq,
    Lt,

    S,
    I,

    B,
    C,

    Cons,
    Car,
    Cdr,
    IsNil,

    Galaxy,
}
