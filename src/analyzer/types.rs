use num_enum::TryFromPrimitive;

pub(crate) const SYMBOLS: &'static [char] = &[
  '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];

pub(crate) const KEYWORDS: &[Keyword] = &[
  Keyword::Class,
  Keyword::Constructor,
  Keyword::Function,
  Keyword::Method,
  Keyword::Field,
  Keyword::Static,
  Keyword::Var,
  Keyword::Int,
  Keyword::Char,
  Keyword::Boolean,
  Keyword::Void,
  Keyword::True,
  Keyword::False,
  Keyword::Null,
  Keyword::This,
  Keyword::Let,
  Keyword::Do,
  Keyword::If,
  Keyword::Else,
  Keyword::While,
  Keyword::Return,
];

#[derive(Debug)]
pub(crate) enum Token<'a> {
  Keyword(Keyword),
  Symbol(Symbol),
  IntegerConstant(u16),
  StringConstant(&'a str),
  Identifier(&'a str),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Keyword {
  Class,
  Constructor,
  Function,
  Method,
  Field,
  Static,
  Var,
  Int,
  Char,
  Boolean,
  Void,
  True,
  False,
  Null,
  This,
  Let,
  Do,
  If,
  Else,
  While,
  Return,
}

impl Keyword {
  pub fn as_str(&self) -> &'static str {
    match self {
      Keyword::Class => "class",
      Keyword::Constructor => "constructor",
      Keyword::Function => "function",
      Keyword::Method => "method",
      Keyword::Field => "field",
      Keyword::Static => "static",
      Keyword::Var => "var",
      Keyword::Int => "int",
      Keyword::Char => "char",
      Keyword::Boolean => "boolean",
      Keyword::Void => "void",
      Keyword::True => "true",
      Keyword::False => "false",
      Keyword::Null => "null",
      Keyword::This => "this",
      Keyword::Let => "let",
      Keyword::Do => "do",
      Keyword::If => "if",
      Keyword::Else => "else",
      Keyword::While => "while",
      Keyword::Return => "return",
    }
  }

  // fn from_str_start(string: &str) -> Option<(Keyword, &str)> {
  //   if string.starts_with(Keyword::Class.as_str()) {
  //     Some(Keyword::Class, &string[Keyword::Class.as_str().len()..])
  //   } else if string.starts_with(Keyword::Constructor.as_str()) {
  //     Some(
  //       Keyword::Constructor,
  //       &string[Keyword::Constructor.as_str().len()..],
  //     )
  //   } else if string.starts_with(Keyword::Function.as_str()) {
  //     Some(
  //       Keyword::Function,
  //       &string[Keyword::Function.as_str().len()..],
  //     )
  //   } else if string.starts_with(Keyword::Method.as_str()) {
  //     Some(Keyword::Method, &string[Keyword::Method.as_str().len()..])
  //   } else if string.starts_with(Keyword::Field.as_str()) {
  //     Some(Keyword::Field, &string[Keyword::Field.as_str().len()..])
  //   } else if string.starts_with(Keyword::Static.as_str()) {
  //     Some(Keyword::Static, &string[Keyword::Static.as_str().len()..])
  //   } else if string.starts_with(Keyword::Var.as_str()) {
  //     Some(Keyword::Var, &string[Keyword::Var.as_str().len()..])
  //   } else if string.starts_with(Keyword::Int.as_str()) {
  //     Some(Keyword::Int, &string[Keyword::Int.as_str().len()..])
  //   } else if string.starts_with(Keyword::Char.as_str()) {
  //     Some(Keyword::Char, &string[Keyword::Char.as_str().len()..])
  //   } else if string.starts_with(Keyword::Boolean.as_str()) {
  //     Some(Keyword::Boolean, &string[Keyword::Boolean.as_str().len()..])
  //   } else if string.starts_with(Keyword::Void.as_str()) {
  //     Some(Keyword::Void, &string[Keyword::Void.as_str().len()..])
  //   } else if string.starts_with(Keyword::True.as_str()) {
  //     Some(Keyword::True, &string[Keyword::True.as_str().len()..])
  //   } else if string.starts_with(Keyword::False.as_str()) {
  //     Some(Keyword::False, &string[Keyword::False.as_str().len()..])
  //   } else if string.starts_with(Keyword::Null.as_str()) {
  //     Some(Keyword::Null, &string[Keyword::Null.as_str().len()..])
  //   } else if string.starts_with(Keyword::This.as_str()) {
  //     Some(Keyword::This, &string[Keyword::This.as_str().len()..])
  //   } else if string.starts_with(Keyword::Let.as_str()) {
  //     Some(Keyword::Let, &string[Keyword::Let.as_str().len()..])
  //   } else if string.starts_with(Keyword::Do.as_str()) {
  //     Some(Keyword::Do, &string[Keyword::Do.as_str().len()..])
  //   } else if string.starts_with(Keyword::If.as_str()) {
  //     Some(Keyword::If, &string[Keyword::If.as_str().len()..])
  //   } else if string.starts_with(Keyword::Else.as_str()) {
  //     Some(Keyword::Else, &string[Keyword::Else.as_str().len()..])
  //   } else if string.starts_with(Keyword::While.as_str()) {
  //     Some(Keyword::While, &string[Keyword::While.as_str().len()..])
  //   } else if string.starts_with(Keyword::Return.as_str()) {
  //     Some(Keyword::Return, &string[Keyword::Return.as_str().len()..])
  //   } else {
  //     None
  //   }
  // }
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum Symbol {
  CurlyOpen = b'{',
  CurlyClose = b'}',
  ParenOpen = b'(',
  ParenClose = b')',
  BracketOpen = b'[',
  BracketClose = b']',
  Period = b'.',
  Comma = b',',
  Semicolon = b';',
  Plus = b'+',
  Minus = b'-',
  Asterix = b'*',
  Slash = b'/',
  Ampersand = b'&',
  VerticalBar = b'|',
  AngleOpen = b'<',
  AngleClose = b'>',
  Equals = b'=',
  Tilde = b'~',
}
