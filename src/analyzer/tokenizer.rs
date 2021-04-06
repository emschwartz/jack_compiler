use super::types::{Symbol, Token, KEYWORDS, SYMBOLS};
use std::convert::TryFrom;
use std::iter::Iterator;
use std::str::FromStr;

pub fn tokenize_lines<'a>(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Token> {
  lines.flat_map(|line| tokenize_line(line).into_iter())
}

pub fn tokenize_line<'a>(line: String) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut start = 0;
  while start < line.len() {
    let substr = &line[start..];
    let next_char = substr.chars().next().unwrap();
    if substr.starts_with("//") {
      // Single-line comment
      break;
    } else if substr.starts_with("/*") {
      // Comment block
      // TODO handle multi-line comments
      let end = substr.find("*/").expect("Unclosed comment");
      start += end + 2;
    } else if next_char == '"' {
      // String constant
      let end = substr.find('"').expect("Unclosed string literal");
      tokens.push(Token::StringConstant(substr[..end].to_string()));
      start += end + 1;
    } else if next_char.is_numeric() {
      // Integer constant
      let end = substr.find(|c: char| !c.is_numeric()).unwrap();
      let int = u16::from_str(&substr[..end]).expect("Cannot parse integer constant");
      tokens.push(Token::IntegerConstant(int));
      start += end;
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
          tokens.push(Token::Identifier(substr[..end].to_string()));
          start += end;
        }
      }
    } else {
      start += 1;
    }
  }
  tokens
}
