use crate::eval::Value;
use std::boxed::Box;

pub fn modulate_list(value: Value) -> String {
    let mut buffer = String::new();
    use crate::eval::Value::*;
    match value {
        Nil => {
            buffer.push_str("00");
            buffer
        },
        Cons(v1, v2) => {
            buffer.push_str("11");
            buffer.push_str(modulate_list(*v1).as_str());
            buffer.push_str(modulate_list(*v2).as_str());
            buffer
        },
        Int(i) => {
            buffer.push_str(modulate(i).as_str());
            buffer
        }
        _ => panic!("Cannot modulate things that are not lists or ints")
    }
}


pub fn modulate(value: i64) -> String {
    let mut buffer = String::new();
    modulate_mut(value, &mut buffer);
    buffer
}

/// https://message-from-space.readthedocs.io/en/latest/message13.html
pub fn modulate_mut(value: i64, buffer: &mut String) {

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

pub fn demodulate(value: &str) -> Value {
    let (ret, _) = demodulate_list(value);
    ret
}

pub fn demodulate_list(value: &str) -> (Value, &str) {
    match &value[0..2] {
        "00" => (Value::Nil, &value[2..]),
        "11" => {
            let (ret, rest) = demodulate_list(&value[2..]);
            let (ret2, rest) = demodulate_list(&rest);
            ( Value::Cons( Box::new(ret), Box::new(ret2)), &rest)
            },
        _ => demodulate_int(value),
    }
}

fn demodulate_int(v: &str) -> (Value, &str) {
    let positive = &v[0..2] == "01";
    let index = v[2..]
        .find('0')
        .expect("Expected '0' in linear-encoded value");
    let length = index * 4;
    let mut final_val = i64::from_str_radix(&v[index+2..index+3+length], 2).unwrap();
    if !positive {
        final_val = -final_val;
    }
    (Value::Int(final_val), &v[index+3+length..])
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
    use crate::eval::Value;
    use crate::eval::eval;
    use crate::parse::exp;
    use crate::lex::lex;

    #[test]
    fn mod_0() {
        assert_eq!(super::modulate(0), "010");
    }

    #[test]
    fn mod_1() {
        assert_eq!(super::modulate(1), "01100001");
    }

    #[test]
    fn mod_16() {
        assert_eq!(super::modulate(16), "0111000010000");
    }

    #[test]
    fn mod_256() {
        assert_eq!(super::modulate(256), "011110000100000000");
    }

    #[test]
    fn mod_neg_100() {
        assert_eq!(super::modulate(-100), "1011001100100");
    }

    // #[test]
    // fn round_trip() {
    //     let mut buffer = String::new();
    //     for value in 0..1000 {
    //         buffer.clear();
    //         super::modulate_mut(value, &mut buffer);
    //         assert_eq!(super::demodulate(&buffer), value);
    //     }
    // }

    #[test]
    fn demodulate_0() {
        assert_eq!(super::demodulate_int("010"), (Value::Int(0), ""));
    }

    #[test]
    fn demodulate_neg_100() {
        assert_eq!(super::demodulate_int("1011001100100"), (Value::Int(-100), ""));
    }

    #[test]
    fn demodulate_16() {
        assert_eq!(super::demodulate_int("0111000010000"), (Value::Int(16), ""));
    }

    #[test]
    fn demodulate_list() {
        let long_list = eval(
            dbg!(&exp(
                &mut lex("ap ap cons 1 ap ap cons ap ap cons 2 ap ap cons 3 nil ap ap cons 4 nil")
                ).expect("bruh")));
        assert_eq!(
            super::demodulate_list("1101100001111101100010110110001100110110010000"),
            (long_list, "")
        )
    }

    #[test]
    fn demodulate() {
        let long_list = eval(
            dbg!(&exp(
                &mut lex("ap ap cons 1 ap ap cons ap ap cons 2 ap ap cons 3 nil ap ap cons 4 nil")
                ).expect("bruh")));
        assert_eq!(
            super::demodulate("1101100001111101100010110110001100110110010000"),
            long_list
        )
    }
}
