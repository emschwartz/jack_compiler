use std::convert::TryFrom;
use crate::ToXml;

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

#[derive(Debug, PartialEq)]
pub enum Token {
  Keyword(Keyword),
  Symbol(Symbol),
  IntegerConstant(u16),
  StringConstant(String),
  Identifier(String),
}

impl ToXml for Token {
  fn to_xml(&self) -> String {
    match self {
      Token::Keyword(keyword) => keyword.to_xml(),
      Token::Symbol(symbol) => symbol.to_xml(),
      Token::IntegerConstant(integer) => format!("<integerConstant> {} </integerConstant>", integer),
      Token::StringConstant(string) => format!("<stringConstant> {} </stringConstant>", string),
      Token::Identifier(identifier) => format!("<identifier> {} </identifier>", identifier)
    }
  }
}

impl ToXml for Vec<Token> {
  fn to_xml(&self) -> String {
      format!("<tokens>
{}
</tokens>", self.iter().map(|t| t.to_xml()).collect::<Vec<String>>().join("\n"))
  }
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

impl ToXml for Keyword {
  fn to_xml(&self) -> String {
      format!("<keyword> {} </keyword>", self.as_str())
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
  LessThan = b'<',
  GreaterThan = b'>',
  Equals = b'=',
  Tilde = b'~',
}

impl TryFrom<char> for Symbol {
  type Error = &'static str;

  fn try_from(c: char) -> Result<Symbol, Self::Error> {
    let symbol = match c {
      '{' => Symbol::CurlyOpen,
      '}' => Symbol::CurlyClose,
      '(' => Symbol::ParenOpen,
      ')' => Symbol::ParenClose,
      '[' => Symbol::BracketOpen,
      ']' => Symbol::BracketClose,
      '.' => Symbol::Period,
      ',' => Symbol::Comma,
      ';' => Symbol::Semicolon,
      '+' => Symbol::Plus,
      '-' => Symbol::Minus,
      '*' => Symbol::Asterix,
      '/' => Symbol::Slash,
      '&' => Symbol::Ampersand,
      '|' => Symbol::VerticalBar,
      '<' => Symbol::LessThan,
      '>' => Symbol::GreaterThan,
      '=' => Symbol::Equals,
      '~' => Symbol::Tilde,
      _ => return Err("Invalid symbol"),
    };
    Ok(symbol)
  }
}

impl ToXml for Symbol {
  fn to_xml(&self) -> String {
    let repr = match self {
      Symbol::LessThan => String::from("&lt;"),
      Symbol::GreaterThan => String::from("&gt;"),
      Symbol::Ampersand => String::from("&amp;"),
      _ => char::from(*self as u8).to_string()
    };
    format!("<symbol> {} </symbol>", repr)
  }
}
