use std::iter::Peekable;

use super::types::*;
use crate::tokenizer::{Keyword, Symbol, Token};

pub fn parse<I: Iterator<Item = Token>>(tokens: I) -> Result<Class, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_class()
}

struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn new(tokens: I) -> Parser<I> {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    fn expect_token(&mut self, token: Token) -> Result<(), String> {
        let next = self
            .tokens
            .next()
            .ok_or(String::from("Unexpected end of input"))?;
        if next == token {
            Ok(())
        } else {
            Err(format!(
                "Expected next token to be: {:?} but got: {:?}",
                token, next
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match self.tokens.next() {
            None => Err(format!("Unexpected end of input, expected identifier")),
            Some(Token::Identifier(identifier)) => Ok(identifier),
            Some(token) => Err(format!(
                "Unexpected token: {:?}, expected identifier",
                token
            )),
        }
    }

    fn expect_var_type(&mut self) -> Result<VarType, String> {
        match self.tokens.next() {
            None => Err(format!("Unexpected end of input, expected var type")),
            Some(Token::Keyword(Keyword::Int)) => Ok(VarType::Int),
            Some(Token::Keyword(Keyword::Char)) => Ok(VarType::Char),
            Some(Token::Keyword(Keyword::Boolean)) => Ok(VarType::Boolean),
            Some(Token::Identifier(class_name)) => Ok(VarType::ClassName(class_name)),
            Some(token) => Err(format!(
                "Unexpected token: {:?}, expected identifier",
                token
            )),
        }
    }

    fn parse_class(&mut self) -> Result<Class, String> {
        self.expect_token(Token::Keyword(Keyword::Class))?;
        let class_name = self.expect_identifier()?;
        self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
        let class_var_declarations = self.parse_class_var_declarations()?;
        let subroutine_declarations = self.parse_subroutine_declarations()?;
        self.expect_token(Token::Symbol(Symbol::CurlyClose))?;

        Ok(Class {
            class_name,
            class_var_declarations,
            subroutine_declarations,
        })
    }

    fn parse_class_var_declarations(&mut self) -> Result<Vec<ClassVarDeclaration>, String> {
        let mut declarations = Vec::new();
        loop {
            match self.tokens.peek() {
                Some(&Token::Keyword(Keyword::Static)) | Some(&Token::Keyword(Keyword::Field)) => {
                    declarations.push(self.parse_class_var_declaration()?)
                }
                _ => break,
            }
        }
        Ok(declarations)
    }

    fn parse_class_var_declaration(&mut self) -> Result<ClassVarDeclaration, String> {
        let static_or_field = match self.tokens.next() {
            Some(Token::Keyword(Keyword::Static)) => StaticOrField::Static,
            Some(Token::Keyword(Keyword::Field)) => StaticOrField::Field,
            _ => return Err(String::from("Expected keyword: 'static' or 'field'")),
        };
        let var_type = self.expect_var_type()?;
        let mut var_names = vec![self.expect_identifier()?];
        while let Some(&Token::Symbol(Symbol::Comma)) = self.tokens.peek() {
            self.expect_token(Token::Symbol(Symbol::Comma))?;
            var_names.push(self.expect_identifier()?);
        }
        self.expect_token(Token::Symbol(Symbol::Semicolon))?;

        Ok(ClassVarDeclaration {
            static_or_field,
            var_type,
            var_names,
        })
    }

    fn parse_subroutine_declarations(&mut self) -> Result<Vec<SubroutineDeclaration>, String> {
        let mut declarations = Vec::new();
        loop {
            let next = self.tokens.peek();
            match next {
                Some(&Token::Keyword(Keyword::Constructor))
                | Some(&Token::Keyword(Keyword::Function))
                | Some(&Token::Keyword(Keyword::Method)) => {
                    declarations.push(self.parse_subroutine_declaration()?)
                }
                _ => break,
            }
        }
        Ok(declarations)
    }

    fn parse_subroutine_declaration(&mut self) -> Result<SubroutineDeclaration, String> {
        let subroutine_type = match self.tokens.next() {
            None => return Err(format!("Unexpected end of input, expected subroutine type")),
            Some(Token::Keyword(Keyword::Constructor)) => SubroutineType::Constructor,
            Some(Token::Keyword(Keyword::Function)) => SubroutineType::Function,
            Some(Token::Keyword(Keyword::Method)) => SubroutineType::Method,
            Some(token) => {
                return Err(format!(
                    "Expected keyword constructor, function or method; got: {:?}",
                    token
                ))
            }
        };

        let return_type = if let Some(&Token::Keyword(Keyword::Void)) = self.tokens.peek() {
            self.tokens.next();
            // None represents void
            None
        } else {
            Some(self.expect_var_type()?)
        };

        let name = self.expect_identifier()?;
        self.expect_token(Token::Symbol(Symbol::ParenOpen))?;
        let parameter_list = self.parse_parameter_list()?;
        self.expect_token(Token::Symbol(Symbol::ParenClose))?;
        let body = self.parse_subroutine_body()?;

        Ok(SubroutineDeclaration {
            subroutine_type,
            return_type,
            name,
            parameter_list,
            body,
        })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, String> {
        let mut parameters = Vec::new();

        match self.tokens.peek() {
            Some(&Token::Keyword(Keyword::Int))
            | Some(&Token::Keyword(Keyword::Char))
            | Some(&Token::Keyword(Keyword::Boolean))
            | Some(&Token::Identifier(_)) => {
                let var_type = self.expect_var_type()?;
                let var_name = self.expect_identifier()?;
                parameters.push((var_type, var_name));

                while self.tokens.peek() == Some(&Token::Symbol(Symbol::Comma)) {
                    self.tokens.next();
                    let var_type = self.expect_var_type()?;
                    let var_name = self.expect_identifier()?;
                    parameters.push((var_type, var_name));
                }
            }
            _ => {}
        };

        Ok(parameters)
    }

    fn parse_subroutine_body(&mut self) -> Result<SubroutineBody, String> {
        self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
        let var_declarations = self.parse_var_declarations()?;
        let statements = self.parse_statements()?;
        self.expect_token(Token::Symbol(Symbol::CurlyClose))?;
        Ok(SubroutineBody {
            var_declarations,
            statements,
        })
    }

    fn parse_var_declarations(&mut self) -> Result<Vec<VarDeclaration>, String> {
        let mut declarations = Vec::new();
        while self.tokens.peek() == Some(&Token::Keyword(Keyword::Var)) {
            self.tokens.next();
            let var_type = self.expect_var_type()?;
            let mut var_names = vec![self.expect_identifier()?];
            while self.tokens.peek() == Some(&Token::Symbol(Symbol::Comma)) {
                self.tokens.next();
                var_names.push(self.expect_identifier()?);
            }
            self.expect_token(Token::Symbol(Symbol::Semicolon))?;

            declarations.push(VarDeclaration {
                var_type,
                var_names,
            })
        }

        Ok(declarations)
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        loop {
            let statement = match self.tokens.peek() {
                Some(Token::Keyword(Keyword::Let)) => Statement::Let(self.parse_let_statement()?),
                Some(Token::Keyword(Keyword::If)) => Statement::If(self.parse_if_statement()?),
                Some(Token::Keyword(Keyword::While)) => {
                    Statement::While(self.parse_while_statement()?)
                }
                Some(Token::Keyword(Keyword::Do)) => Statement::Do(self.parse_do_statement()?),
                Some(Token::Keyword(Keyword::Return)) => {
                    Statement::Return(self.parse_return_statement()?)
                }
                _ => break,
            };
            statements.push(statement);
        }

        Ok(statements)
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, String> {
        self.expect_token(Token::Keyword(Keyword::Let))?;
        let var_name = self.expect_identifier()?;

        let left_side_expression =
            if self.tokens.peek() == Some(&Token::Symbol(Symbol::BracketOpen)) {
                self.tokens.next();
                let expression = self.parse_expression()?;
                self.expect_token(Token::Symbol(Symbol::BracketClose))?;
                Some(expression)
            } else {
                None
            };

        self.expect_token(Token::Symbol(Symbol::Equals))?;
        let right_side_expression = self.parse_expression()?;
        self.expect_token(Token::Symbol(Symbol::Semicolon))?;

        Ok(LetStatement {
            var_name,
            left_side_expression,
            right_side_expression,
        })
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement, String> {
        self.expect_token(Token::Keyword(Keyword::If))?;
        self.expect_token(Token::Symbol(Symbol::ParenOpen))?;
        let expression = self.parse_expression()?;
        self.expect_token(Token::Symbol(Symbol::ParenClose))?;
        self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
        let if_statements = self.parse_statements()?;
        self.expect_token(Token::Symbol(Symbol::CurlyClose))?;

        let else_statements = if self.tokens.peek() == Some(&Token::Keyword(Keyword::Else)) {
            self.tokens.next();
            self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
            let statements = self.parse_statements()?;
            self.expect_token(Token::Symbol(Symbol::CurlyClose))?;
            Some(statements)
        } else {
            None
        };

        Ok(IfStatement {
            expression,
            if_statements,
            else_statements,
        })
    }

    fn parse_while_statement(&mut self) -> Result<WhileStatement, String> {
        self.expect_token(Token::Keyword(Keyword::While))?;
        self.expect_token(Token::Symbol(Symbol::ParenOpen))?;
        let expression = self.parse_expression()?;
        self.expect_token(Token::Symbol(Symbol::ParenClose))?;
        self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
        let statements = self.parse_statements()?;
        self.expect_token(Token::Symbol(Symbol::CurlyClose))?;
        Ok(WhileStatement {
            expression,
            statements,
        })
    }

    fn parse_do_statement(&mut self) -> Result<DoStatement, String> {
        self.expect_token(Token::Keyword(Keyword::Do))?;
        let identifier = self.expect_identifier()?;
        let subroutine_call = self.parse_subroutine_call(identifier)?;
        self.expect_token(Token::Symbol(Symbol::Semicolon))?;
        Ok(DoStatement(subroutine_call))
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, String> {
        self.expect_token(Token::Keyword(Keyword::Return))?;
        let expression = match self.tokens.peek() {
            Some(Token::Symbol(Symbol::Semicolon)) => None,
            Some(_) => Some(self.parse_expression()?),
            None => {
                return Err(format!(
                    "Unexpected end of input, expected ';' or expression"
                ))
            }
        };
        self.expect_token(Token::Symbol(Symbol::Semicolon))?;
        Ok(ReturnStatement(expression))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let term = self.parse_term()?;
        let mut ops = Vec::new();
        loop {
            match self.tokens.peek() {
                Some(&Token::Symbol(Symbol::Plus)) => {
                    self.tokens.next();
                    ops.push((Op::Plus, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::Minus)) => {
                    self.tokens.next();
                    ops.push((Op::Minus, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::Asterix)) => {
                    self.tokens.next();
                    ops.push((Op::Asterix, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::Slash)) => {
                    self.tokens.next();
                    ops.push((Op::Slash, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::Ampersand)) => {
                    self.tokens.next();
                    ops.push((Op::Ampersand, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::VerticalBar)) => {
                    self.tokens.next();
                    ops.push((Op::VerticalBar, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::LessThan)) => {
                    self.tokens.next();
                    ops.push((Op::LessThan, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::GreaterThan)) => {
                    self.tokens.next();
                    ops.push((Op::GreaterThan, self.parse_term()?))
                }
                Some(&Token::Symbol(Symbol::Equals)) => {
                    self.tokens.next();
                    ops.push((Op::Equals, self.parse_term()?))
                }
                _ => break,
            }
        }
        Ok(Expression { term, ops })
    }

    fn parse_term(&mut self) -> Result<Term, String> {
        let next = self
            .tokens
            .next()
            .ok_or(String::from("Unexpected end of input, expected term"))?;
        let term = match next {
            // integerConstant
            Token::IntegerConstant(int) => Term::IntegerConstant(int),
            // stringConstant
            Token::StringConstant(string) => Term::StringConstant(string),
            // keywordConstant
            Token::Keyword(Keyword::True) => Term::KeywordConstant(KeywordConstant::True),
            Token::Keyword(Keyword::False) => Term::KeywordConstant(KeywordConstant::False),
            Token::Keyword(Keyword::Null) => Term::KeywordConstant(KeywordConstant::Null),
            Token::Keyword(Keyword::This) => Term::KeywordConstant(KeywordConstant::This),
            // different possibilities:
            Token::Identifier(var_name) => match self.tokens.peek() {
              // varName[expression]
              Some(&Token::Symbol(Symbol::BracketOpen)) => {
                self.tokens.next();
                let expression = self.parse_expression()?;
                self.expect_token(Token::Symbol(Symbol::BracketClose))?;
                Term::VarNameExpression((var_name, Box::new(expression)))
              }
              // subroutineName(expressionList)
              Some(&Token::Symbol(Symbol::ParenOpen))
              // classOrVarName.subroutineName
              | Some(&Token::Symbol(Symbol::Period)) => Term::SubroutineCall(self.parse_subroutine_call(var_name)?),
              // varName
              _ => Term::VarName(var_name)
            },
            // (expression)
            Token::Symbol(Symbol::ParenOpen) => {
                let expression = self.parse_expression()?;
                self.expect_token(Token::Symbol(Symbol::ParenClose))?;
                Term::Expression(Box::new(expression))
            }
            // unaryOp term
            Token::Symbol(Symbol::Minus) => {
                Term::UnaryOpTerm((UnaryOp::Minus, Box::new(self.parse_term()?)))
            }
            Token::Symbol(Symbol::Tilde) => {
                Term::UnaryOpTerm((UnaryOp::Tilde, Box::new(self.parse_term()?)))
            }
            token => return Err(format!("Unexpected token: {:?}, expected term", token)),
        };
        Ok(term)
    }

    fn parse_subroutine_call(&mut self, identifier: String) -> Result<SubroutineCall, String> {
        let (class_or_var_name, subroutine_name) = match self.tokens.next() {
            Some(Token::Symbol(Symbol::ParenOpen)) => (None, identifier),
            Some(Token::Symbol(Symbol::Period)) => {
                let subroutine_name = self.expect_identifier()?;
                self.expect_token(Token::Symbol(Symbol::ParenOpen))?;
                let class_or_var_name = identifier;
                (Some(class_or_var_name), subroutine_name)
            }
            Some(token) => {
                return Err(format!(
                    "Unexpected token: {:?}, expected '(' or '.' for subroutine call",
                    token
                ))
            }
            None => {
                return Err(String::from(
                    "Unexpected end of input, expected subroutine call",
                ))
            }
        };

        // expression list
        let expression_list = if self.tokens.peek() == Some(&Token::Symbol(Symbol::ParenClose)) {
            self.tokens.next();
            Vec::new()
        } else {
            let mut expression_list = vec![self.parse_expression()?];
            while self.tokens.peek() == Some(&Token::Symbol(Symbol::Comma)) {
                self.tokens.next();
                expression_list.push(self.parse_expression()?);
            }
            self.expect_token(Token::Symbol(Symbol::ParenClose))?;
            expression_list
        };
        Ok(SubroutineCall {
            class_or_var_name,
            subroutine_name,
            expression_list,
        })
    }
}
