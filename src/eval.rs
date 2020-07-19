use std::rc::Rc;

use crate::Client;
use crate::ast::Atom;
use crate::ast::AtomCache;
use crate::ast::Exp;
use crate::ast::Protocol;
use crate::draw;

pub fn interact(
    c: &Client,
    p: &Protocol,
    a: &mut AtomCache,

    s: Rc<Exp>,
    v: Rc<Exp>,
) -> Rc<Exp> {
    let e = eval(
        &Exp::app(Exp::app(Rc::clone(&p[p.galaxy]), s), v),
        p,
        a,
    );

    _interact(c, p, a, e)
}

fn _interact(
    c: &Client,
    p: &Protocol,
    a: &mut AtomCache,
    e: Rc<Exp>,
) -> Rc<Exp> {
    let (flag, tail) = e.to_cons();
    let (state, tail) = tail.to_cons();
    let (data, tail) = tail.to_cons();

    assert_eq!(**tail, Exp::Atom(Atom::Nil));

    if let Exp::Atom(Atom::Int(0)) = &**flag {
        draw::multidraw(&data);
        Rc::clone(state)
    } else {
        let new_data = c
            .send_alien_message(a, data)
            .expect("Failed to send message to server");
        interact(c, p, a, Rc::clone(state), new_data)
    }
}

pub fn eval(e: &Rc<Exp>, p: &Protocol, a: &mut AtomCache) -> Rc<Exp> {
    if let Some(cached) = e.get_cached() {
        return cached;
    }

    let mut prev = Rc::clone(&e);

    loop {
        let next = step(&prev, p, a);

        // Found fixpoint of `step` function
        if prev == next {
            e.set_cached(Rc::clone(&next));
            return next;
        }

        prev = next;
    }
}

fn step(e: &Rc<Exp>, p: &Protocol, a: &mut AtomCache) -> Rc<Exp> {
    if let Some(cached) = e.get_cached() {
        return cached;
    }

    // Evaluate atoms:
    //
    // ```text
    // a
    // ```
    let (f, x) = match &**e {
    | Exp::Atom(Atom::Var(var)) => return Rc::clone(&p[*var]),
    | Exp::Atom(atom) => return a.get(*atom),
    | Exp::App(f, x, _) => (f, x),
    };

    let f = eval(f, p, a);

    // Evaluate single-argument functions:
    //
    // ```text
    //   app
    //  /   \
    // f     x
    // ```
    let (f, x, y) = match &*f {
    | Exp::Atom(Atom::Neg) => {
        let x = -eval(x, p, a).to_int();
        return a.get(Atom::Int(x));
    }
    | Exp::Atom(Atom::Inc) => {
        let x = eval(x, p, a).to_int();
        let y = 1;
        return a.get(Atom::Int(x + y))
    }
    | Exp::Atom(Atom::Dec) => {
        let x = eval(x, p, a).to_int();
        let y = 1;
        return a.get(Atom::Int(x - y))
    }
    | Exp::Atom(Atom::I) => return Rc::clone(x),
    | Exp::Atom(Atom::Nil) => return a.get(Atom::Bool(true)),
    | Exp::Atom(Atom::IsNil) => return Exp::app(
        Rc::clone(x),
        Exp::app(
            a.get(Atom::Bool(true)),
            Exp::app(
                a.get(Atom::Bool(true)),
                a.get(Atom::Bool(false)),
            ),
        ),
    ),
    | Exp::Atom(Atom::Car) => return Exp::app(
        Rc::clone(x),
        a.get(Atom::Bool(true)),
    ),
    | Exp::Atom(Atom::Cdr) => return Exp::app(
        Rc::clone(x),
        a.get(Atom::Bool(false)),
    ),
    | Exp::Atom(_) => return Rc::clone(e),
    // Note: application is on the left, so we swap arguments
    | Exp::App(f, y, _) => (f, y, x),
    };

    let f = eval(f, p, a);

    // Evaluate two-argument functions:
    //
    // ```text
    //      app
    //     /   \
    //   app    y
    //  /   \
    // f    x
    // ```
    let (f, x, y, z) = match &*f {
    | Exp::Atom(Atom::Bool(true)) => return Rc::clone(x),
    | Exp::Atom(Atom::Bool(false)) => return Rc::clone(y),
    | Exp::Atom(Atom::Add) => {
        let x = eval(x, p, a).to_int();
        let y = eval(y, p, a).to_int();
        return a.get(Atom::Int(x + y));
    }
    | Exp::Atom(Atom::Mul) => {
        let x = eval(x, p, a).to_int();
        let y = eval(y, p, a).to_int();
        return a.get(Atom::Int(x * y));
    }
    | Exp::Atom(Atom::Div) => {
        let x = eval(x, p, a).to_int();
        let y = eval(y, p, a).to_int();
        return a.get(Atom::Int(x / y));
    }
    | Exp::Atom(Atom::Lt) => {
        let x = eval(x, p, a).to_int();
        let y = eval(y, p, a).to_int();
        return a.get(Atom::Bool(x < y));
    }
    | Exp::Atom(Atom::Eq) => {
        let x = eval(x, p, a).to_int();
        let y = eval(y, p, a).to_int();
        return a.get(Atom::Bool(x == y));
    }
    | Exp::Atom(Atom::Cons) => {
        let cons = Exp::app(
            Exp::app(
                a.get(Atom::Cons),
                eval(x, p, a),
            ),
            eval(y, p, a),
        );
        cons.set_cached(Rc::clone(&cons));
        return cons;
    }
    | Exp::Atom(_) => return Rc::clone(e),
    | Exp::App(f, z, _) => (f, z, x, y),
    };

    let f = eval(f, p, a);

    // Evaluate three-argument functions:
    //
    // ```text
    //         app
    //        /   \
    //      app    z
    //     /   \
    //   app    y
    //  /   \
    // f     x
    // ```
    match &*f {
    | Exp::Atom(Atom::S) => Exp::app(
        Exp::app(Rc::clone(x), Rc::clone(z)),
        Exp::app(Rc::clone(y), Rc::clone(z)),
    ),
    | Exp::Atom(Atom::C) => Exp::app(
        Exp::app(Rc::clone(x), Rc::clone(z)),
        Rc::clone(y),
    ),
    | Exp::Atom(Atom::B) => Exp::app(
        Rc::clone(x),
        Exp::app(Rc::clone(y), Rc::clone(z)),
    ),
    | Exp::Atom(Atom::Cons) => Exp::app(
        Exp::app(Rc::clone(z), Rc::clone(x)),
        Rc::clone(y),
    ),
    | _ => Rc::clone(e),
    }
}
