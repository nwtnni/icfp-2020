use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ops;
use std::rc::Rc;

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

    pub fn cons<L, R>(lhs: L, rhs: R) -> Rc<Exp>
    where L: Into<Rc<Exp>>,
          R: Into<Rc<Exp>>,
    {
        Self::app(Self::app(Exp::Atom(Atom::Cons), lhs), rhs)
    }

    pub fn get_cached(&self) -> Option<Rc<Exp>> {
        match self {
        | Exp::Atom(_) => None,
        | Exp::App(_, _, cache) => cache.borrow().clone(),
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
        | other => panic!(format!("Expected `int`, but found: {}", other)),
        }
    }

    /// Asserts that `self` is in the following tree shape, and extracts sub-trees `h`, and `t`:
    ///
    /// ```text
    ///      app
    ///     /   \
    ///    app   t
    ///   /   \
    /// cons   h
    /// ```
    pub fn to_cons(&self) -> (&Rc<Exp>, &Rc<Exp>) {
        let (cons_h, t) = match self {
        | Exp::Atom(atom) => panic!(format!("Expected `ap ap cons <CAR> <CDR>`, but found: {}", atom)),
        | Exp::App(cons_h, t, _) => (cons_h, t),
        };

        let (cons, h) = match &**cons_h {
        | Exp::Atom(atom) => panic!(format!("Expected `ap cons <CAR>`, but found: {}", atom)),
        | Exp::App(cons, h, _) => (cons, h),
        };

        match &**cons {
        | Exp::Atom(Atom::Cons) => (h, t),
        | other => panic!(format!("Expected `cons`, but found: {}", other)),
        }
    }
}

impl PartialEq for Exp {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
        | (Exp::Atom(lhs), Exp::Atom(rhs)) => lhs == rhs,
        | (Exp::App(llhs, lrhs, _), Exp::App(rlhs, rrhs, _)) => llhs == rlhs && lrhs == rrhs,
        | _ => false,
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

impl From<Atom> for Exp {
    fn from(atom: Atom) -> Self {
        Exp::Atom(atom)
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
        | Exp::Atom(atom) => write!(fmt, "{}", atom),
        | Exp::App(f, x, _) => write!(fmt, "ap {} {}", f, x),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
        | Atom::Nil => write!(fmt, "nil"),
        | Atom::Int(int) => write!(fmt, "{}", int),
        | Atom::Var(var) => write!(fmt, "{}", var),
        | Atom::Bool(bool) => write!(fmt, "{}", bool),
        | Atom::Neg => write!(fmt, "neg"),
        | Atom::Inc => write!(fmt, "inc"),
        | Atom::Dec => write!(fmt, "dec"),
        | Atom::Add => write!(fmt, "add"),
        | Atom::Mul => write!(fmt, "mul"),
        | Atom::Div => write!(fmt, "div"),
        | Atom::Eq => write!(fmt, "eq"),
        | Atom::Lt => write!(fmt, "lt"),
        | Atom::S => write!(fmt, "s"),
        | Atom::I => write!(fmt, "i"),
        | Atom::B => write!(fmt, "b"),
        | Atom::C => write!(fmt, "c"),
        | Atom::Cons => write!(fmt, "cons"),
        | Atom::Car => write!(fmt, "car"),
        | Atom::Cdr => write!(fmt, "cdr"),
        | Atom::IsNil => write!(fmt, "isNil"),
        | Atom::Galaxy => write!(fmt, "galaxy"),
        }
    }
}
