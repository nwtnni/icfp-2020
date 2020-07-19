use std::fmt;
use std::sync::Arc;

use crate::ast;
use crate::Client;
use crate::draw;
use crate::PROTOCOL;
use crate::transport;

pub enum Value {
    Int(i64),
    Bool(bool),
    Cons(Box<Value>, Box<Value>),
    Var(u64),
    Nil,
    Closure(Box<dyn Fn(&Arc<ast::Exp>) -> Value>),
}

impl Value {
    fn to_exp(&self) -> ast::Exp {
        match self {
        | Value::Int(int) => ast::Exp::Int(*int),
        | Value::Bool(bool) => ast::Exp::Bool(*bool),
        | Value::Var(var) => ast::Exp::Var(*var),
        | Value::Nil => ast::Exp::Nil,
        | Value::Cons(head, tail) => {
            ast::Exp::App(
                Arc::new(ast::Exp::App(
                    Arc::new(ast::Exp::Cons),
                    Arc::new(head.to_exp()),
                )),
                Arc::new(tail.to_exp()),
            )
        }
        | Value::Closure(_) => panic!("Cannot convert closure to expression"),
        }
    }
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
        | Closure(_) => write!(f, "<closure>"),
        | Int(n) => write!(f, "{}", n),
        | Bool(n) => write!(f, "{}", n),
        | Cons(n1, n2) => f.debug_tuple("")
            .field(n1)
            .field(n2)
            .finish(),
        | Var(n) => write!(f, ":{}", n),
        | Nil => write!(f, "()"),
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

#[allow(dead_code)]
pub fn interact(
    client: &Client,
    state: Value,
    vector: Value,
) -> Value {
    step(
        client,
        eval(
            &ast::Exp::App(
                Arc::new(ast::Exp::App(
                    Arc::clone(&PROTOCOL[PROTOCOL.galaxy]),
                    Arc::new(state.to_exp()),
                )),
                Arc::new(vector.to_exp()),
            ),
        ),
    )
}

fn step(
    client: &Client,
    list: Value,
) -> Value {
    if let Value::Cons(flag, tail) = list {
    if let Value::Cons(state, tail) = *tail {
    if let Value::Cons(data, tail) = *tail {
    if let Value::Nil = *tail {
        log::debug!("Flag: {:?}", &flag);
        log::debug!("State: {:?}", &state);
        log::debug!("Data: {:?}", &data);
        if let Value::Int(0) = *flag {
            draw::multidraw(&data);
            return *state;
        } else {
            return interact(
                client,
                *state,
                client
                    .send_alien_message(transport::modulate_list(*data))
                    .expect("Failed to send message to server"),
            );
        }
    }}}}
    panic!("Invalid arguments to `step`");
}

pub fn eval(expr: &ast::Exp) -> Value {

    use ast::Exp::*;

    log::debug!("Evaluating: {:?}", expr);

    match expr {
    | Nil => Value::Nil,
    | Int(n) => Value::Int(*n),
    | Var(v) => eval(&PROTOCOL[*v]),
    | Bool(b) => Value::Bool(*b),
    | App(f, v) => dbg!(closure(eval(&f))(&v)),
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
    | value => panic!(format!("Expected closure or bool but got: {:?}", value)),
    }
}
