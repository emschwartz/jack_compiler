use super::types::{Symbol, Token, KEYWORDS, SYMBOLS};
use std::convert::TryFrom;
use std::iter::Iterator;
use std::str::FromStr;

pub fn tokenize<'a>(lines: impl Iterator<Item = &'a str>) -> impl Iterator<Item = Token<'a>> {
  lines.flat_map(|line| {
    let mut tokens = Vec::new();
    let mut start = 0;
    while start < line.len() {
      let substr = &line[start..];
      let next_char = substr.chars().next().unwrap();
      if next_char == '"' {
        // String constant
        let end = substr.find('"').expect("Unclosed string literal");
        tokens.push(Token::StringConstant(&substr[..end]));
        start = end + 1;
      } else if next_char.is_numeric() {
        // Integer constant
        let end = substr.find(|c: char| !c.is_numeric()).unwrap();
        let int = u16::from_str(&substr[..end]).expect("Cannot parse integer constant");
        tokens.push(Token::IntegerConstant(int));
        start = end;
      } else if SYMBOLS.contains(&substr.chars().next().unwrap()) {
        // Symbol
        tokens.push(Token::Symbol(Symbol::try_from(next_char as u8).unwrap()));
        start += 1;
      } else if !next_char.is_whitespace() {
        for variant in KEYWORDS {
          // Keyword
          if substr.starts_with(variant.as_str()) {
            tokens.push(Token::Keyword(*variant));
            start += variant.as_str().len();
          } else {
            // Identifier
            let end = substr
              .find(|c: char| !c.is_alphanumeric() && c != '_')
              .unwrap_or(substr.len());
            tokens.push(Token::Identifier(&substr[..end]));
            start = end;
          }
        }
      }
    }
    tokens.into_iter()
  })
}
