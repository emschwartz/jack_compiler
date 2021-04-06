use num_enum::TryFromPrimitive;

pub const SYMBOLS: &'static [char] = &[
  '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];
pub const KEYWORDS: &[Keyword] = &[
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
pub enum Token {
  Keyword(Keyword),
  Symbol(Symbol),
  IntegerConstant(u16),
  StringConstant(String),
  Identifier(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
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
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Symbol {
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
