#[derive(Clone, Debug)]
pub struct Program<'a> {
    pub stms: Vec<Stm<'a>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Stm<'a> {
    pub var: u64,
    pub exp: Exp<'a>,
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
