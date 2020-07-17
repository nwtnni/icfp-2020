#[derive(Copy, Clone, Debug)]
pub enum Token {
  /// Arbitrary variable.
  ///
  /// - Type: int
  /// - Parity: 0
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message8.html
  Var(i64),

  /// Literal integer.
  ///
  /// - Type: int
  /// - Parity: 0
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message2.html
  Int(i64),

  /// Church-encoded boolean.
  ///
  /// - Type
  ///   - t: 'a -> 'b -> 'a
  ///   - f: 'a -> 'b -> 'b
  /// - Parity: 2
  /// - Reference: https://en.wikipedia.org/wiki/Church_encoding#Church_Booleans
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message11.html
  Bool(bool),

  /// Integer increment.
  ///
  /// - Type: inc: int -> int
  /// - Parity: 1
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message5.html
  Inc,

  /// Integer decrement.
  ///
  /// - Type: dec: int -> int
  /// - Parity: 1
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message6.html
  Dec,

  /// Integer negation.
  ///
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message16.html
  Neg,

  /// `n`th power of 2.
  ///
  /// - Type: pwr2: int -> int
  /// - Parity: 1
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message23.html
  Pow2,

  /// Integer sum.
  ///
  /// - Type: sum: int -> int -> int
  /// - Parity: 2
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message7.html
  Add,

  /// Integer multiply.
  ///
  /// - Type: mul: int -> int -> int
  /// - Parity: 2
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message9.html
  Mul,

  /// Integer division (round toward 0).
  ///
  /// - Type: div: int -> int -> int
  /// - Parity: 2
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message10.html
  Div,

  /// Integer (?) equality.
  ///
  /// - Type: int -> int -> bool (?)
  /// - Parity: 2
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message11.html
  Eq,

  /// Strict integer less-than.
  ///
  /// - Type: int -> int -> bool (?)
  /// - Parity: 2
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message12.html
  Lt,

  /// Modulate.
  ///
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message13.html
  Mod,

  /// Demodulate.
  ///
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message14.html
  Dem,

  /// Variable assignment.
  Assign,

  /// Send to server (?).
  ///
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message15.html
  Send,

  /// Function application.
  ///
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message17.html
  App,

  /// S combinator.
  ///
  /// - Parity: 3
  /// - Reference: https://en.wikipedia.org/wiki/SKI_combinator_calculus
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message18.html
  S,

  /// C combinator.
  ///
  /// - Parity: 3
  /// - Reference: https://en.wikipedia.org/wiki/B,_C,_K,_W_system
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message19.html
  C,

  /// B combinator.
  ///
  /// - Parity: 3
  /// - Reference: https://en.wikipedia.org/wiki/B,_C,_K,_W_system
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message20.html
  B,

  /// K combinator.
  ///
  /// - Parity: 2
  /// - Reference: https://en.wikipedia.org/wiki/SKI_combinator_calculus
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message21.html
  K,

  /// I combinator.
  ///
  /// - Parity: 1
  /// - Reference: https://en.wikipedia.org/wiki/SKI_combinator_calculus
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message24.html
  I,

  /// Lisp `cons`.
  ///
  /// - Reference: https://en.wikipedia.org/wiki/Cons
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message25.html
  Cons,

  /// Lisp `car`.
  ///
  /// - Reference: https://en.wikipedia.org/wiki/CAR_and_CDR
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message26.html
  Car,

  /// Lisp `cdr`.
  ///
  /// - Reference: https://en.wikipedia.org/wiki/CAR_and_CDR
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message27.html
  Cdr,

  /// Empty list.
  ///
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message28.html
  Nil,

  /// Check for empty list.
  ///
  /// - Source: https://message-from-space.readthedocs.io/en/latest/message29.html
  IsNil,

  /// - Source: https://message-from-space.readthedocs.io/en/latest/message42.html
  Galaxy,
}
