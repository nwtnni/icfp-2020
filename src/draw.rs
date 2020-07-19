use std::io;
use std::io::Write;
use std::time;
use std::thread;

use crate::eval::Value;

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

fn extract_int(v: &Value) -> i64 {
    match v {
    | Value::Int(vv) => *vv,
    | _ => panic!("Extracting int from non-int"),
    }
}

fn draw_point(v: &Value) {
    match v {
    | Value::Cons(v1, v2) => draw_at(extract_int(v1), extract_int(v2)),
    | _ => panic!("Not a valid pair")
    }
}

fn _draw(v: &Value) {
    match v {
    | Value::Cons(v, n) => {
        draw_point(v);
        _draw(n)
    },
    | Value::Nil => (),
    | _ => panic!("Not a valid list"),
    }
}

pub fn draw(v: &Value) {
    clear();
    _draw(v);
    io::stdout().flush().unwrap();
}

fn _multidraw(v: &Value) {
    match v {
    | Value::Cons(image, rest) => {
        _draw(image);
        thread::sleep(time::Duration::from_secs(1));
        _multidraw(rest);
    },
    | Value::Nil => (),
    | _ => panic!("Not a valid list of images"),
    }
}

pub fn multidraw(v: &Value) {
    alt_buffer();
    hide_cursor();
    clear();
    _multidraw(v);
    show_cursor();
    reg_buffer();
    io::stdout().flush().unwrap();
}
