use typed_arena::Arena;

use crate::ast;

pub fn eval<'arena>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    expr: &ast::Exp<'arena>,
) -> ast::Exp<'arena> {
    use ast::Exp::*;
    match *expr {
    | Nil => Nil,
    | Int(n) => Int(n),
    | Var(name) => Var(name),
    | Bool(b) => Bool(b),
    | App(Neg, e) => Int(-eval_int(arena, e)),
    | App(App(Add, e1), e2) => {
        let i1: i64 = eval_int(arena, e1);
        let i2: i64 = eval_int(arena, e2);
        Int(i1 + i2)
    }
    | App(App(Mul, e1), e2) => {
        let i1: i64 = eval_int(arena, e1);
        let i2: i64 = eval_int(arena, e2);
        Int(i1 * i2)
    }
    | _ => todo!("Add more cases")
    }
}

fn eval_int<'arena>(
    arena: &'arena Arena<ast::Exp<'arena>>,
    expr: &ast::Exp<'arena>,
) -> i64 {
    use ast::Exp::*;
    match eval(arena, expr) {
    | Int(n) => n,
    | _ => panic!("Expected int"),
    }
}
