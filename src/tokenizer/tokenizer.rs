use super::types::{Symbol, Token, KEYWORDS, SYMBOLS};
use std::convert::TryFrom;
use std::iter::Iterator;
use std::str::FromStr;

const MAX_INT: u16 = 32767;

pub fn tokenize<'a>(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Token> {
    let mut is_comment_block = false;
    lines.flat_map(move |line| {
        let mut tokens = Vec::new();
        let mut start = 0;

        // Go through each character in the line
        while start < line.len() {
            let substr = &line[start..];
            let next_char = substr.chars().next().unwrap();

            // Handle comment blocks
            if is_comment_block {
                // End of comment block
                if substr.starts_with("*/") {
                    is_comment_block = false;
                    start += 2;
                } else {
                    // Still in comment block, ignore this character
                    start += 1;
                }
            } else if substr.starts_with("/*") {
                // Comment block
                is_comment_block = true;
                start += 2;
            } else if substr.starts_with("//") {
                // Single-line comment
                break;
            } else if next_char == '"' {
                // String constant
                let end = 1 + &substr[1..].find('"').expect("Unclosed string literal");
                tokens.push(Token::StringConstant(substr[1..end].to_string()));
                start += end + 1;
            } else if next_char.is_numeric() {
                // Integer constant
                let end = 1 + &substr[1..].find(|c: char| !c.is_numeric()).unwrap();
                let int = u16::from_str(&substr[..end]).expect("Cannot parse integer constant");
                if int > MAX_INT {
                    panic!("Integer exceeds max value");
                }
                tokens.push(Token::IntegerConstant(int));
                start += end;
            } else if SYMBOLS.contains(&next_char) {
                // Symbol
                tokens.push(Token::Symbol(Symbol::try_from(next_char).unwrap()));
                start += 1;
            } else if !next_char.is_whitespace() {
                let mut is_keyword = false;
                for variant in KEYWORDS {
                    // Keyword
                    if substr.starts_with(variant.as_ref()) {
                        // Also check that the next character after isn't alphabetic
                        // (so we don't accidentally treat "double" as the "do" keyword)
                        let next_char = substr.chars().nth(variant.as_ref().len());
                        if let Some(next_char) = next_char {
                            if !next_char.is_alphabetic() {
                                tokens.push(Token::Keyword(*variant));
                                start += variant.as_ref().len();
                                is_keyword = true;
                                break;
                            }
                        }
                    }
                }
                // Identifier
                if !is_keyword {
                    let end = 1 + &substr[1..]
                        .find(|c: char| !c.is_alphanumeric() && c != '_')
                        .unwrap_or(substr.len() - 1);
                    tokens.push(Token::Identifier(substr[..end].to_string()));
                    start += end;
                }
            } else {
                start += 1;
            }
        }
        tokens.into_iter()
    })
}
