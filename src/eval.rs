use std::fmt;
use std::rc::Rc;

use crate::ast;

pub enum Value {
    Int(i64),
    Bool(bool),
    Cons(Rc<Value>, Rc<Value>),
    Var(u64),
    Nil,
    Closure(Rc<dyn Fn(&ast::Exp) -> Value>),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        use Value::*;
        match self {
        | Closure(_) => panic!("Cannot clone closure"),
        | Int(n) => Int(*n),
        | Bool(n) => Bool(*n),
        | Cons(a, b) => Cons(Rc::clone(a), Rc::clone(b)),
        | Var(n) => Var(*n),
        | Nil => Nil,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
        | Closure(_) => panic!("Cannot clone closure"),
        | Int(n) => write!(f, "Int: {}", n),
        | Bool(n) => write!(f, "Bool: {}", n),
        | Cons(n1, n2) => write!(f, "Cons: ( {:?}, {:?} )", n1, n2),
        | Var(n) => write!(f, "Var: {}", n),
        | Nil => write!(f, "Nil"),
        }
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
        | (Closure(_), _)
        | (_, Closure(_)) => panic!("Cannot clone closure"),
        | (Int(n1), Int(n2)) => n1 == n2,
        | (Bool(n1), Bool(n2)) => n1 == n2,
        | (Cons(n1, m1), Cons(n2, m2)) => n1 == n2 && m1 == m2,
        | (Var(n1), Var(n2)) => n1 == n2,
        | (Nil, Nil) => true,
        | _ => false,
        }
    }
}

pub fn eval(expr: &ast::Exp, protocol: &Rc<ast::Protocol>) -> Value {

    use ast::Exp::*;
    match expr {
    | Nil => Value::Nil,
    | Int(n) => Value::Int(*n),
    | Var(v) => Value::Var(*v),
    | Bool(b) => Value::Bool(*b),
    | App(f, v) =>
        match eval(&f, protocol) {
        | Value::Closure(func) => func(&v),
        | Value::Bool(true) => todo!("hmm"),
        | _ => panic!("Can't apply values that are not closures or bool")
        },
    | Neg => Value::Closure(Rc::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Int(n) => Value::Int(-n),
            | _ => panic!("Expected int as argument for Neg"),
            }
        }
    })),
    | IsNil => Value::Closure(Rc::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Nil => Value::Bool(true),
            | Value::Cons(_, _) => Value::Bool(false),
            | _ => panic!("Expected Nil or Cons as argument for IsNil"),
            }
        }
    })),
    | Add => Value::Closure(Rc::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Int(n1) => Value::Closure(Rc::new({
                let protocol = Rc::clone(&protocol);
                move |e| {
                    match eval(e, &protocol) {
                    | Value::Int(n2) => Value::Int(n1 + n2),
                    | _ => panic!("Expected int as arg1 for Add"),
                    }
                }
            })),
            | _ => panic!("Expected int as arg2 for Add"),
            }
        }
    })),
    | Mul => Value::Closure(Rc::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Int(n1) => Value::Closure(Rc::new({
                let protocol = Rc::clone(&protocol);
                move |e| {
                    match eval(e, &protocol) {
                    | Value::Int(n2) => Value::Int(n1 * n2),
                    | _ => panic!("Expected int as arg1 for Mul"),
                    }
                }
            })),
            | _ => panic!("Expected int as arg2 for Mul"),
            }
        }
    })),
    | Div => Value::Closure(Rc::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Int(n1) => Value::Closure(Rc::new({
                let protocol = Rc::clone(&protocol);
                move |e| {
                    match eval(e, &protocol) {
                    | Value::Int(n2) => Value::Int(n1 / n2),
                    | _ => panic!("Expected int as arg1 for Div"),
                    }
                }
            })),
            | _ => panic!("Expected int as arg2 for Div"),
            }
        }
    })),
    | Eq => Value::Closure(Rc::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Int(n1) => Value::Closure(Rc::new({
                let protocol = Rc::clone(&protocol);
                move |e| {
                    match eval(e, &protocol) {
                    | Value::Int(n2) => Value::Bool(n1 == n2),
                    | _ => panic!("Expected int as arg1 for Eq"),
                    }
                }
            })),
            | _ => panic!("Expected int as arg2 for Eq"),
            }
        }
    })),
    | Lt => Value::Closure(Rc::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Int(n1) => Value::Closure(Rc::new({
                let protocol = Rc::clone(&protocol);
                move |e| {
                    match eval(e, &protocol) {
                    | Value::Int(n2) => Value::Bool(n1 < n2),
                    | _ => panic!("Expected int as arg1 for Lt"),
                    }
                }
            })),
            | _ => panic!("Expected int as arg2 for Lt"),
            }
        }
    })),
    | _ => todo!(),
    }
}
