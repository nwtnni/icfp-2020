use std::rc::Rc;

use crate::ast::Atom;
use crate::ast::AtomCache;
use crate::ast::Exp;

pub fn modulate(list: &Exp) -> String {
    let mut buffer = String::new();
    modulate_mut(list, &mut buffer);
    buffer
}

fn modulate_mut(list: &Exp, buffer: &mut String) {
    if let Exp::Atom(Atom::Nil) = list {
        buffer.push_str("00");
        return;
    }

    if let Exp::Atom(Atom::Int(int)) = list {
        modulate_int_mut(*int, buffer);
        return;
    }

    let (head, tail) = list.to_cons();
    buffer.push_str("11");
    modulate_mut(&head, buffer);
    modulate_mut(&tail, buffer);
}

pub fn modulate_int(value: i64) -> String {
    let mut buffer = String::new();
    modulate_int_mut(value, &mut buffer);
    buffer
}

/// https://message-from-space.readthedocs.io/en/latest/message13.html
pub fn modulate_int_mut(value: i64, buffer: &mut String) {

    // Bits 0..1 define a positive or negative number (and signal width)
    // via a high/low or low/high signal change:
    // - 01: positive number
    // - 10: negative number
    if value >= 0 {
        buffer.push_str("01");
    } else {
        buffer.push_str("10");
    }

    // Bits 2..(n+2) define the width of the following binary-encoded
    // number via a unary-encoded number of length n composed of high
    // signals ending with a low signal. The number width (in bits) is
    // four times the unary encoding (i.e. 4 * n):
    //
    // - 0: 0 [i.e. the number zero]
    // - 10: 4-bit number [i.e. 1-7]
    // - 110: 8-bit number [i.e. 1-255]
    // - 1110: 12-bit number [i.e. 1-4095]
    let width = (67 - value.abs().leading_zeros()) / 4;
    for _ in 0..width {
        buffer.push('1');
    }
    buffer.push('0');

    // Edge case: 0 doesn't have any bits to encode
    if width == 0 {
        return;
    }

    // The remaining bits, i.e. (n + 3)..(n + 3 + 4*n - 1), determine
    // the number itself, in most-significant-bit first binary notation.
    // Using the examples from this message:
    // - 0001: 1
    // - 00010000: 16
    // - 000100000000: 256
    let bits = value.abs() as u64;
    let mut mask = 1 << (width * 4 - 1);

    for _ in 0..width * 4 {
        match bits & mask > 0 {
        | false => buffer.push('0'),
        | true => buffer.push('1'),
        }
        mask >>= 1;
    }
}

pub fn demodulate(list: &str, cache: &mut AtomCache) -> Rc<Exp> {
    let (ret, _) = demodulate_list(list, cache);
    ret.set_cached(Rc::clone(&ret));
    ret
}

pub fn demodulate_list<'v>(value: &'v str, cache: &mut AtomCache) -> (Rc<Exp>, &'v str) {
    match &value[0..2] {
    | "00" => (cache.get(Atom::Nil), &value[2..]),
    | "11" => {
        let (head, rest) = demodulate_list(&value[2..], cache);
        let (tail, rest) = demodulate_list(&rest, cache);
        (Exp::app(Exp::app(cache.get(Atom::Cons), head), tail), rest)
    }
    | _ => demodulate_int(value, cache),
    }
}

fn demodulate_int<'v>(v: &'v str, cache: &mut AtomCache) -> (Rc<Exp>, &'v str) {
    let positive = &v[0..2] == "01";
    let index = v[2..]
        .find('0')
        .expect("Expected '0' in linear-encoded value");
    let length = index * 4;
    let mut final_val = i64::from_str_radix(&v[index+2..index+3+length], 2).unwrap();
    if !positive {
        final_val = -final_val;
    }
    (cache.get(Atom::Int(final_val)), &v[index+3+length..])
}

/// https://message-from-space.readthedocs.io/en/latest/message14.html
fn demodulate_number(value: &str) -> i64 {
    let positive = &value[0..2] == "01";

    // Note: +2 necessary since index is w.r.t. [2..]
    let index = 2 + value[2..]
        .find('0')
        .expect("Expected '0' in linear-encoded value");

    i64::from_str_radix(&value[index..], 2)
        .map(|value| if positive { value } else { -value })
        .expect("Expected valid binary string in linear-encoded value")
}

#[cfg(test)]
mod tests {

    // use crate::parse::exp;
    // use crate::lex::lex;

    // #[test]
    // fn mod_0() {
    //     assert_eq!(super::modulate(0), "010");
    // }

    // #[test]
    // fn mod_1() {
    //     assert_eq!(super::modulate(1), "01100001");
    // }

    // #[test]
    // fn mod_16() {
    //     assert_eq!(super::modulate(16), "0111000010000");
    // }

    // #[test]
    // fn mod_256() {
    //     assert_eq!(super::modulate(256), "011110000100000000");
    // }

    // #[test]
    // fn mod_neg_100() {
    //     assert_eq!(super::modulate(-100), "1011001100100");
    // }

    // // #[test]
    // // fn round_trip() {
    // //     let mut buffer = String::new();
    // //     for value in 0..1000 {
    // //         buffer.clear();
    // //         super::modulate_mut(value, &mut buffer);
    // //         assert_eq!(super::demodulate(&buffer), value);
    // //     }
    // // }

    // #[test]
    // fn demodulate_0() {
    //     assert_eq!(super::demodulate_int("010"), (Value::Int(0), ""));
    // }

    // #[test]
    // fn demodulate_neg_100() {
    //     assert_eq!(super::demodulate_int("1011001100100"), (Value::Int(-100), ""));
    // }

    // #[test]
    // fn demodulate_16() {
    //     assert_eq!(super::demodulate_int("0111000010000"), (Value::Int(16), ""));
    // }

    // #[test]
    // fn demodulate_list() {
    //     let long_list = eval(
    //         dbg!(&exp(
    //             &mut lex("ap ap cons 1 ap ap cons ap ap cons 2 ap ap cons 3 nil ap ap cons 4 nil")
    //             ).expect("bruh")));
    //     assert_eq!(
    //         super::demodulate_list("1101100001111101100010110110001100110110010000"),
    //         (long_list, "")
    //     )
    // }

    // #[test]
    // fn demodulate() {
    //     let long_list = eval(
    //         dbg!(&exp(
    //             &mut lex("ap ap cons 1 ap ap cons ap ap cons 2 ap ap cons 3 nil ap ap cons 4 nil")
    //             ).expect("bruh")));
    //     assert_eq!(
    //         super::demodulate("1101100001111101100010110110001100110110010000"),
    //         long_list
    //     )
    // }
}
