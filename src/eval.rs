use std::fmt;
use std::sync::Arc;

use crate::ast;

pub enum Value {
    Int(i64),
    Bool(bool),
    Cons(Box<Value>, Box<Value>),
    Var(u64),
    Nil,
    Closure(Box<dyn Fn(&Arc<ast::Exp>) -> Value>),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        use Value::*;
        match self {
        | Closure(_) => panic!("Cannot clone closure"),
        | Int(n) => Int(*n),
        | Bool(n) => Bool(*n),
        | Cons(h, t) => Cons(Box::clone(h), Box::clone(t)),
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

pub fn eval(expr: &ast::Exp) -> Value {

    use ast::Exp::*;

    match expr {
    | Nil => Value::Nil,
    | Int(n) => Value::Int(*n),
    | Var(v) => Value::Var(*v),
    | Bool(b) => Value::Bool(*b),
    | App(f, v) => closure(eval(&f))(&v),
    | Neg => Value::Closure(Box::new(|e| Value::Int(-int(eval(e))))),
    | Inc => Value::Closure(Box::new(|e| Value::Int(int(eval(e)) + 1))),
    | Dec => Value::Closure(Box::new(|e| Value::Int(int(eval(e)) - 1))),
    | IsNil => Value::Closure(Box::new(|e| {
        match eval(e) {
        | Value::Nil => Value::Bool(true),
        | Value::Cons(_, _) => Value::Bool(false),
        | _ => panic!("Expected Nil or Cons as argument for IsNil"),
        }
    })),
    | Add => Value::Closure(Box::new({
        |e1| {
            Value::Closure(Box::new({
                let e1 = Arc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1));
                    let n2 = int(eval(e2));
                    Value::Int(n1 + n2)
                }
            }))
        }
    })),
    | Mul => Value::Closure(Box::new({
        |e1| {
            Value::Closure(Box::new({
                let e1 = Arc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1));
                    let n2 = int(eval(e2));
                    Value::Int(n1 * n2)
                }
            }))
        }
    })),
    | Div => Value::Closure(Box::new({
        |e1| {
            Value::Closure(Box::new({
                let e1 = Arc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1));
                    let n2 = int(eval(e2));
                    Value::Int(n1 / n2)
                }
            }))
        }
    })),
    | Eq => Value::Closure(Box::new({
        |e1| {
            Value::Closure(Box::new({
                let e1 = Arc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1));
                    let n2 = int(eval(e2));
                    Value::Bool(n1 == n2)
                }
            }))
        }
    })),
    | Lt => Value::Closure(Box::new({
        |e1| {
            Value::Closure(Box::new({
                let e1 = Arc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1));
                    let n2 = int(eval(e2));
                    Value::Bool(n1 < n2)
                }
            }))
        }
    })),
    | Cons => Value::Closure(Box::new({
        |head| {
            Value::Closure(Box::new({
                let head = Arc::clone(head);
                move |tail| {
                    let head = Box::new(eval(&head));
                    let tail = Box::new(eval(tail));
                    Value::Cons(head, tail)
                }
            }))
        }
    })),
    | Car => Value::Closure(Box::new(|list| { let (head, _) = cons(eval(list)); head })),
    | Cdr => Value::Closure(Box::new(|list| { let (tail, _) = cons(eval(list)); tail })),
    | S => Value::Closure(Box::new({
        |x0| {
            Value::Closure(Box::new({
                let x0 = Arc::clone(x0);
                move |x1| {
                    Value::Closure(Box::new({
                        let x0 = Arc::clone(&x0);
                        let x1 = Arc::clone(x1);
                        move |x2| {
                            let f = (closure(eval(&x0)))(&x2);
                            (closure(f))(&Arc::new(App(Arc::clone(&x1), Arc::clone(&x2))))
                        }
                    }))
                }
            }))
        }
    })),
    | I => Value::Closure(Box::new(|x| eval(x))),
    | B => Value::Closure(Box::new({
        |x0| {
            Value::Closure(Box::new({
                let x0 = Arc::clone(x0);
                move |x1| {
                    Value::Closure(Box::new({
                        let x0 = Arc::clone(&x0);
                        let x1 = Arc::clone(x1);
                        move |x2| {
                            (closure(eval(&x0)))(&Arc::new(App(Arc::clone(&x1), Arc::clone(&x2))))
                        }
                    }))
                }
            }))
        }
    })),
    | C => Value::Closure(Box::new({
        |x0| {
            Value::Closure(Box::new({
                let x0 = Arc::clone(x0);
                move |x1| {
                    Value::Closure(Box::new({
                        let x0 = Arc::clone(&x0);
                        let x1 = Arc::clone(x1);
                        move |x2| {
                            (closure((closure(eval(&x0)))(&x2)))(&x1)
                        }
                    }))
                }
            }))
        }
    })),
    | _ => todo!(),
    }
}

fn int(value: Value) -> i64 {
    match value {
    | Value::Int(int) => int,
    | _ => panic!("Expected int"),
    }
}

fn cons(value: Value) -> (Value, Value) {
    match value {
    | Value::Cons(head, tail) => (*head, *tail),
    | _ => panic!("Expected cons"),
    }
}

fn closure(value: Value) -> Box<dyn Fn(&Arc<ast::Exp>) -> Value> {
    match value {
    | Value::Closure(func) => func,
    | Value::Bool(true) => {
        Box::new(|lhs| {
            let lhs = Arc::clone(&lhs);
            Value::Closure(Box::new(move |_| eval(&lhs)))
        })
    }
    | Value::Bool(false) => Box::new(|_| Value::Closure(Box::new(|rhs| eval(&rhs)))),
    | _ => panic!("Expected closure or bool")
    }
}
