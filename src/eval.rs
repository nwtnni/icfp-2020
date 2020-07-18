use std::fmt;
use std::rc::Rc;

use crate::ast;

pub enum Value {
    Int(i64),
    Bool(bool),
    Cons(Box<Value>, Box<Value>),
    Var(u64),
    Nil,
    Closure(Box<dyn Fn(&Rc<ast::Exp>) -> Value>),
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
        | Value::Bool(true) => {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                move |lhs| {
                    Value::Closure(Box::new({
                        let protocol = Rc::clone(&protocol);
                        let lhs = Rc::clone(&lhs);
                        move |_| {
                            eval(&lhs, &protocol)
                        }
                    }))
                }
            }))
        }
        | Value::Bool(false) => {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                move |_| {
                    Value::Closure(Box::new({
                        let protocol = Rc::clone(&protocol);
                        move |rhs| {
                            eval(&rhs, &protocol)
                        }
                    }))
                }
            }))
        }
        | _ => panic!("Can't apply values that are not closures or bool")
        },
    | Neg => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |e| Value::Int(-int(eval(e, &protocol)))
    })),
    | IsNil => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |e| {
            match eval(e, &protocol) {
            | Value::Nil => Value::Bool(true),
            | Value::Cons(_, _) => Value::Bool(false),
            | _ => panic!("Expected Nil or Cons as argument for IsNil"),
            }
        }
    })),
    | Add => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |e1| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let e1 = Rc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1, &protocol));
                    let n2 = int(eval(e2, &protocol));
                    Value::Int(n1 + n2)
                }
            }))
        }
    })),
    | Mul => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |e1| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let e1 = Rc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1, &protocol));
                    let n2 = int(eval(e2, &protocol));
                    Value::Int(n1 * n2)
                }
            }))
        }
    })),
    | Div => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |e1| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let e1 = Rc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1, &protocol));
                    let n2 = int(eval(e2, &protocol));
                    Value::Int(n1 / n2)
                }
            }))
        }
    })),
    | Eq => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |e1| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let e1 = Rc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1, &protocol));
                    let n2 = int(eval(e2, &protocol));
                    Value::Bool(n1 == n2)
                }
            }))
        }
    })),
    | Lt => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |e1| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let e1 = Rc::clone(&e1);
                move |e2| {
                    let n1 = int(eval(&e1, &protocol));
                    let n2 = int(eval(e2, &protocol));
                    Value::Bool(n1 < n2)
                }
            }))
        }
    })),
    | Cons => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |head| {
            Value::Closure(Box::new({
                let head = Rc::clone(head);
                let protocol = Rc::clone(&protocol);
                move |tail| {
                    let head = Box::new(eval(&head, &protocol));
                    let tail = Box::new(eval(tail, &protocol));
                    Value::Cons(head, tail)
                }
            }))
        }
    })),
    | Car => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |list| {
            let (head, _) = cons(eval(list, &protocol));
            head
        }
    })),
    | Cdr => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |list| {
            let (tail, _) = cons(eval(list, &protocol));
            tail
        }
    })),
    | S => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |x0| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let x0 = Rc::clone(x0);
                move |x1| {
                    Value::Closure(Box::new({
                        let protocol = Rc::clone(&protocol);
                        let x0 = Rc::clone(&x0);
                        let x1 = Rc::clone(x1);
                        move |x2| {
                            let f = (closure(eval(&x0, &protocol)))(&x2);
                            (closure(f))(&Rc::new(App(Rc::clone(&x1), Rc::clone(&x2))))
                        }
                    }))
                }
            }))
        }
    })),
    | I => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |x| eval(x, &protocol)
    })),
    | B => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |x0| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let x0 = Rc::clone(x0);
                move |x1| {
                    Value::Closure(Box::new({
                        let protocol = Rc::clone(&protocol);
                        let x0 = Rc::clone(&x0);
                        let x1 = Rc::clone(x1);
                        move |x2| {
                            (closure(eval(&x0, &protocol)))(&Rc::new(App(Rc::clone(&x1), Rc::clone(&x2))))
                        }
                    }))
                }
            }))
        }
    })),
    | C => Value::Closure(Box::new({
        let protocol = Rc::clone(&protocol);
        move |x0| {
            Value::Closure(Box::new({
                let protocol = Rc::clone(&protocol);
                let x0 = Rc::clone(x0);
                move |x1| {
                    Value::Closure(Box::new({
                        let protocol = Rc::clone(&protocol);
                        let x0 = Rc::clone(&x0);
                        let x1 = Rc::clone(x1);
                        move |x2| {
                            (closure((closure(eval(&x0, &protocol)))(&x2)))(&x1)
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

fn closure(value: Value) -> Box<dyn Fn(&Rc<ast::Exp>) -> Value> {
    match value {
    | Value::Closure(closure) => closure,
    | _ => panic!("Expected closure"),
    }
}
