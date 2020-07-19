use std::io;
use std::io::Write;
use std::time;
use std::thread;

use crate::ast::Atom;
use crate::ast::Exp;

const CSI: &str = "\x1b[";
const CLEAR_SCREEN: &str = "\x1b[2J";

fn hide_cursor() {
    print!("{}?25l", CSI);
}

fn show_cursor() {
    print!("{}?25h", CSI);
}

fn alt_buffer() {
    print!("{}?1049h", CSI);
}

fn reg_buffer() {
    print!("{}?1049l", CSI);
}

fn clear() {
    print!("{}", CLEAR_SCREEN);
}

fn draw_at(x: i64, y:i64) {
    print!("{}{};{}H", CSI, x+1, y+1);
    print!("█");
    io::stdout().flush().unwrap();
}

fn extract_int(exp: &Exp) -> i64 {
    match exp {
    | Exp::Atom(Atom::Int(vv)) => *vv,
    | _ => panic!("Extracting int from non-int"),
    }
}

fn draw_point(exp: &Exp) {
    let (v1, v2) = exp.to_cons();
    draw_at(extract_int(v1), extract_int(v2));
}

fn _draw(exp: &Exp) {
    if let Exp::Atom(Atom::Nil) = exp {
        return;
    }

    let (v, n) = exp.to_cons();
    draw_point(v);
    _draw(n);
}

pub fn draw(exp: &Exp) {
    clear();
    _draw(exp);
    io::stdout().flush().unwrap();
}

fn _multidraw(exp: &Exp) {
    if let Exp::Atom(Atom::Nil) = exp {
        return;
    }

    let (image, rest) = exp.to_cons();
    _draw(image);
    thread::sleep(time::Duration::from_secs(1));
    _multidraw(rest);
}

pub fn multidraw(exp: &Exp) {
    alt_buffer();
    hide_cursor();
    clear();
    _multidraw(exp);
    show_cursor();
    reg_buffer();
    io::stdout().flush().unwrap();
}

fn draw_point_exp(exp: &Exp, frame: &mut Vec<(i64, i64)>) {
    let (x, y) = exp.to_cons();
    frame.push((extract_int(x), extract_int(y)));
}

fn draw_exp(exp: &Exp, frame: &mut Vec<(i64, i64)>) {
    if let Exp::Atom(Atom::Nil) = exp {
        return;
    }

    let (point, rest) = exp.to_cons();
    draw_point_exp(point, frame);
    draw_exp(rest, frame);
}

pub fn multidraw_exp(exp: &Exp, buffer: &mut Vec<Vec<(i64, i64)>>) {
    if let Exp::Atom(Atom::Nil) = exp {
        return;
    }

    let (points, rest) = exp.to_cons();
    let mut frame = Vec::new();
    draw_exp(points, &mut frame);
    buffer.push(frame);
    multidraw_exp(rest, buffer);
}
