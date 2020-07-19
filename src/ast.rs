use std::sync::Arc;
use std::collections::HashMap;
use std::ops;

/// Interaction protocol.
#[derive(Clone, Debug, Default)]
pub struct Protocol {
    pub assignments: Arc<HashMap<u64, Arc<Exp>>>,
    pub galaxy: u64,
}

impl ops::Index<u64> for Protocol {
    type Output = Arc<Exp>;
    fn index(&self, var: u64) -> &Self::Output {
        &self.assignments[&var]
    }
}

/// Test suite.
#[derive(Clone, Debug)]
pub struct TestSuite {
    pub equals: Vec<Test>,
}

#[derive(Clone, Debug)]
pub struct Test {
    pub assignments: Arc<HashMap<u64, Arc<Exp>>>,
    pub equal: Equal
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
    Inc,
    Dec,

    Add,
    Mul,
    Div,

    Eq,
    Lt,

    App(Arc<Exp>, Arc<Exp>),

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
