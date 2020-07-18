use std::rc::Rc;
use std::collections::HashMap;

/// Interaction protocol.
#[derive(Clone, Debug)]
pub struct Protocol {
    pub assignments: Rc<HashMap<u64, Rc<Exp>>>,
    pub galaxy: u64,
}

/// Test suite.
#[derive(Clone, Debug)]
pub struct TestSuite {
    pub equals: Vec<Equal>,
}

#[derive(Clone, Debug)]
pub struct Equal {
    pub lhs: Exp,
    pub rhs: Exp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Exp {
    Nil,
    Int(i64),
    Var(u64),
    Bool(bool),

    Neg,

    Add,
    Mul,
    Div,

    Eq,
    Lt,

    App(Rc<Exp>, Rc<Exp>),

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
