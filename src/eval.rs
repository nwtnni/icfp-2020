use std::rc::Rc;

use crate::ast::Atom;
use crate::ast::AtomCache;
use crate::ast::Exp;
use crate::ast::Protocol;

fn eval(e: &Rc<Exp>, p: &Protocol, a: &mut AtomCache) -> Rc<Exp> {
    if let Some(cached) = e.get_cached() {
        return Rc::clone(cached);
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
        return Rc::clone(cached);
    }

    // Evaluate atoms:
    //
    // ```text
    // a
    // ```
    let (f, x) = match &**e {
    | Exp::Atom(atom) => return a.get(*atom),
    | Exp::App(f, x, _) => (f, x),
    };

    // Evaluate single-argument functions:
    //
    // ```text
    //   app
    //  /   \
    // f     x
    // ```
    let (f, x, y) = match &**f {
    | Exp::Atom(Atom::Neg) => return a.get(Atom::Int(-eval(x, p, a).to_int())),
    | Exp::Atom(Atom::Inc) => return a.get(Atom::Int(eval(x, p, a).to_int() + 1)),
    | Exp::Atom(Atom::Dec) => return a.get(Atom::Int(eval(x, p, a).to_int() - 1)),
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
    | Exp::Atom(atom) => panic!(format!(
        "Expected `neg`, `i`, `nil`, `isnil`, `car`, or `cdr`, but found: {:?}",
        atom
    )),
    // Note: application is on the left, so we swap arguments
    | Exp::App(f, y, _) => (f, y, x),
    };

    // Evaluate two-argument functions:
    //
    // ```text
    //      app
    //     /   \
    //   app    y
    //  /   \
    // f    x
    // ```
    let (f, x, y, z) = match &**f {
    | Exp::Atom(Atom::Bool(true)) => return Rc::clone(x),
    | Exp::Atom(Atom::Bool(false)) => return Rc::clone(y),
    | Exp::Atom(Atom::Add) => return a.get(Atom::Int(eval(x, p, a).to_int() + eval(y, p, a).to_int())),
    | Exp::Atom(Atom::Mul) => return a.get(Atom::Int(eval(x, p, a).to_int() * eval(y, p, a).to_int())),
    | Exp::Atom(Atom::Div) => return a.get(Atom::Int(eval(x, p, a).to_int() / eval(y, p, a).to_int())),
    | Exp::Atom(Atom::Lt) => return a.get(Atom::Bool(eval(x, p, a).to_int() < eval(y, p, a).to_int())),
    | Exp::Atom(Atom::Eq) => return a.get(Atom::Bool(eval(x, p, a).to_int() == eval(y, p, a).to_int())),
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
    | Exp::Atom(atom) => panic!(format!(
        "Expected `t`, `f`, `add`, `mul`, `div`, `lt`, `eq`, or `cons`, but found: {:?}",
        atom,
    )),
    | Exp::App(f, z, _) => (f, z, x, y),
    };

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
    match &**f {
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
    | other => panic!(format!(
        "Expected `s`, `c`, `b`, or `cons`, but found: {:?}",
        other
    )),
    }
}
