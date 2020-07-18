/// Interaction protocol.
#[derive(Clone, Debug)]
pub struct Protocol<'a> {
    pub assignments: Vec<Assign<'a>>,
    pub galaxy: u64,
}

/// Test suite.
#[derive(Clone, Debug)]
pub struct TestSuite<'a> {
    pub equals: Vec<Equal<'a>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Assign<'a> {
    pub var: u64,
    pub exp: Exp<'a>,
}

#[derive(Copy, Clone, Debug)]
pub struct Equal<'a> {
    pub lhs: Exp<'a>,
    pub rhs: Exp<'a>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Exp<'a> {
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

    App(&'a Exp<'a>, &'a Exp<'a>),

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
