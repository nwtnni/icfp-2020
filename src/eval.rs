use crate::ast;

pub fn eval(
    expr: &ast::Exp,
) -> ast::Exp {
    use ast::Exp::*;
    match *expr {
    | Nil => Nil,
    | Int(n) => Int(n),
    | Var(name) => Var(name),
    | Bool(b) => Bool(b),
    | _ => todo!("Add more cases")
    }
}
