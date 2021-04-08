use std::iter::Peekable;

use crate::tokenizer::{Token, Keyword, Symbol};
use super::types::*;

pub fn parse<I: Iterator<Item = Token>>(tokens: I) -> Result<Class, String> {
  let mut parser = Parser::new(tokens);
  parser.compile_class()
}

struct Parser<I: Iterator<Item = Token>> {
  tokens: Peekable<I>,
}

impl<I> Parser<I>
where I: Iterator<Item = Token>
 {
  fn new(tokens: I) -> Parser<I> {
    Parser {
      tokens: tokens.peekable()
    }
  }

  fn expect_token(&mut self, token: Token) -> Result<(), String> {
    let next = self.tokens.next().ok_or(String::from("Unexpected end of input"))?;
    if next == token {
      Ok(())
    } else {
      Err(format!("Expected next token to be: {:?} but got: {:?}", token, next))
    }
  }

  fn expect_identifier(&mut self) -> Result<String, String> {
    match self.tokens.next() {
      None => Err(format!("Unexpected end of input, expected identifier")),
      Some(Token::Identifier(identifier)) => Ok(identifier),
      Some(token) => Err(format!("Unexpected token: {:?}, expected identifier", token ))
    }
  }

  fn expect_var_type(&mut self) -> Result<VarType, String> {
    match self.tokens.next() {
      None => Err(format!("Unexpected end of input, expected var type")),
      Some(Token::Keyword(Keyword::Int)) => Ok(VarType::Int),
      Some(Token::Keyword(Keyword::Char)) => Ok(VarType::Char),
      Some(Token::Keyword(Keyword::Boolean)) => Ok(VarType::Boolean),
      Some(Token::Identifier(class_name)) => Ok(VarType::ClassName(class_name)),
      Some(token) => Err(format!("Unexpected token: {:?}, expected identifier", token ))
    }
  }

  fn compile_class(&mut self) -> Result<Class, String> {
    self.expect_token(Token::Keyword(Keyword::Class))?;
    let class_name = self.expect_identifier()?;
    dbg!("class", &class_name);
    self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
    let class_var_declarations = self.compile_class_var_declarations()?;
    let subroutine_declarations = self.compile_subroutine_declarations()?;
    self.expect_token(Token::Symbol(Symbol::CurlyClose))?;

    Ok(Class {
      class_name,
      class_var_declarations,
      subroutine_declarations
    })
  }

  fn compile_class_var_declarations(&mut self) -> Result<Vec<ClassVarDeclaration>, String> {
    let mut declarations = Vec::new();
    loop {
    match self.tokens.peek() {
      Some(&Token::Keyword(Keyword::Static)) | Some(&Token::Keyword(Keyword::Field)) => declarations.push(self.compile_class_var_declaration()?),
        _ => break,
      }
    }
    Ok(declarations)
  }

  fn compile_class_var_declaration(&mut self) -> Result<ClassVarDeclaration, String> {
    let static_or_field = match self.tokens.next() {
      Some(Token::Keyword(Keyword::Static)) => StaticOrField::Static,
      Some(Token::Keyword(Keyword::Field)) => StaticOrField::Field,
      _ => return Err(String::from("Expected keyword: 'static' or 'field'")),
    };
    dbg!("class var declaration", static_or_field);
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
      var_names
    })
  }

  fn compile_subroutine_declarations(&mut self) -> Result<Vec<SubroutineDeclaration>, String> {
    dbg!("subroutines");
    let mut declarations = Vec::new();
    loop {
      let next = self.tokens.peek();
      match next {
        Some(&Token::Keyword(Keyword::Constructor))
        |  Some(&Token::Keyword(Keyword::Function))
        |  Some(&Token::Keyword(Keyword::Method)) => declarations.push(self.compile_subroutine_declaration()?),
        _ => break,
      }
    }
    Ok(declarations)
  }

  fn compile_subroutine_declaration(&mut self) -> Result<SubroutineDeclaration, String> {
    let subroutine_type = match self.tokens.next() {
      None => return Err(format!("Unexpected end of input, expected subroutine type")),
      Some(Token::Keyword(Keyword::Constructor)) => SubroutineType::Constructor,
      Some(Token::Keyword(Keyword::Function)) => SubroutineType::Function,
      Some(Token::Keyword(Keyword::Method)) => SubroutineType::Method,
      Some(token) => return Err(format!("Expected keyword constructor, function or method; got: {:?}", token))
    };

    let return_type = if let Some(&Token::Keyword(Keyword::Void)) = self.tokens.peek() {
      self.tokens.next();
      // None represents void
      None
    } else {
      Some(self.expect_var_type()?)
    };

    let name = self.expect_identifier()?;
    dbg!("subroutine", &name);
    self.expect_token(Token::Symbol(Symbol::ParenOpen))?;
    let parameter_list = self.compile_parameter_list()?;
    self.expect_token(Token::Symbol(Symbol::ParenClose))?;
    let body = self.compile_subroutine_body()?;

    Ok(SubroutineDeclaration {
      subroutine_type,
      return_type,
      name,
      parameter_list,
      body
    })
  }

  fn compile_parameter_list(&mut self) -> Result<Vec<Parameter>, String> {
    let mut parameters = Vec::new();

    match self.tokens.peek() {
      Some(&Token::Keyword(Keyword::Int)) |
      Some(&Token::Keyword(Keyword::Char)) |
      Some(&Token::Keyword(Keyword::Boolean)) |
      Some(&Token::Identifier(_)) => {
        let var_type = self.expect_var_type()?;
        let var_name = self.expect_identifier()?;
        parameters.push((var_type, var_name));

        while self.tokens.peek() == Some(&Token::Symbol(Symbol::Comma)) {
          self.tokens.next();
          let var_type = self.expect_var_type()?;
          let var_name = self.expect_identifier()?;
          parameters.push((var_type, var_name));
        }
      },
      _ => {}
    };

    Ok(parameters)
  }

  fn compile_subroutine_body(&mut self) -> Result<SubroutineBody, String> {
    dbg!("subroutine body");
    self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
    let var_declarations = self.compile_var_declarations()?;
    let statements = self.compile_statements()?;
    self.expect_token(Token::Symbol(Symbol::CurlyClose))?;
    Ok(SubroutineBody {
      var_declarations,
      statements
    })
  }

  fn compile_var_declarations(&mut self) -> Result<Vec<VarDeclaration>, String> {
    dbg!("var delcarations");
    let mut declarations = Vec::new();
    while self.tokens.peek() == Some(&Token::Keyword(Keyword::Var)) {
      self.tokens.next();
      let var_type = self.expect_var_type()?;
      let mut var_names = vec![self.expect_identifier()?];
      while self.tokens.peek() == Some(&Token::Symbol(Symbol::Comma)) {
        self.tokens.next();
        var_names.push(self.expect_identifier()?);
      }

      declarations.push(VarDeclaration {
        var_type,
        var_names
      })
    }

    Ok(declarations)
  }

  fn compile_statements(&mut self) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    dbg!("statements");

    loop {
      let statement = match self.tokens.peek() {
        Some(Token::Keyword(Keyword::Let)) => Statement::Let(self.compile_let_statement()?),
        Some(Token::Keyword(Keyword::If)) => Statement::If(self.compile_if_statement()?),
        Some(Token::Keyword(Keyword::While)) => Statement::While(self.compile_while_statement()?),
        Some(Token::Keyword(Keyword::Do)) => Statement::Do(self.compile_do_statement()?),
        Some(Token::Keyword(Keyword::Return)) => Statement::Return(self.compile_return_statement()?),
        _ => break,
      };
      statements.push(statement);
    }

    Ok(statements)
  }

  fn compile_let_statement(&mut self) -> Result<LetStatement, String> {
    dbg!("let");
    self.expect_token(Token::Keyword(Keyword::Let))?;
    let var_name = self.expect_identifier()?;

    let left_side_expression = if self.tokens.peek() == Some(&Token::Symbol(Symbol::BracketOpen)) {
      self.tokens.next();
      let expression = self.compile_expression()?;
      self.expect_token(Token::Symbol(Symbol::BracketClose))?;
      Some(expression)
    } else {
      None
    };

    self.expect_token(Token::Symbol(Symbol::Equals))?;
    let right_side_expression = self.compile_expression()?;
    self.expect_token(Token::Symbol(Symbol::Semicolon))?;

    Ok(LetStatement {
      var_name,
      left_side_expression,
      right_side_expression
    })
  }

  fn compile_if_statement(&mut self) -> Result<IfStatement, String> {
    dbg!("if");
    self.expect_token(Token::Keyword(Keyword::If))?;
    self.expect_token(Token::Symbol(Symbol::ParenOpen))?;
    let expression = self.compile_expression()?;
    self.expect_token(Token::Symbol(Symbol::ParenClose))?;
    self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
    let if_statements = self.compile_statements()?;
    self.expect_token(Token::Symbol(Symbol::CurlyClose))?;

    let else_statements = if self.tokens.peek() == Some(&Token::Keyword(Keyword::Else)) {
      self.tokens.next();
      self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
      let statements = self.compile_statements()?;
      self.expect_token(Token::Symbol(Symbol::CurlyClose))?;
      statements
    } else {
      Vec::new()
    };

    Ok(IfStatement {
      expression,
      if_statements,
      else_statements
    })
  }

  fn compile_while_statement(&mut self) -> Result<WhileStatement, String> {
    dbg!("while");
    self.expect_token(Token::Keyword(Keyword::While))?;
    self.expect_token(Token::Symbol(Symbol::ParenOpen))?;
    let expression = self.compile_expression()?;
    self.expect_token(Token::Symbol(Symbol::ParenClose))?;
    self.expect_token(Token::Symbol(Symbol::CurlyOpen))?;
    let statements = self.compile_statements()?;
    self.expect_token(Token::Symbol(Symbol::CurlyClose))?;
    Ok(WhileStatement {
      expression,
      statements
    })
  }

  fn compile_do_statement(&mut self) -> Result<DoStatement, String> {
    dbg!("do");
    self.expect_token(Token::Keyword(Keyword::Do))?;
    Ok(DoStatement(self.compile_subroutine_call()?))
  }

  fn compile_return_statement(&mut self) -> Result<ReturnStatement, String> {
    dbg!("return");
    self.expect_token(Token::Keyword(Keyword::Return))?;
    let expression = match self.tokens.peek() {
      Some(Token::Symbol(Symbol::Semicolon)) => None,
      Some(_) => Some(self.compile_expression()?),
      None => return Err(format!("Unexpected end of input, expected ';' or expression"))
    };
    Ok(ReturnStatement(expression))
  }

  fn compile_expression(&mut self) -> Result<Expression, String> {
    unimplemented!()
  }

  fn compile_subroutine_call(&mut self) -> Result<SubroutineCall, String> {
    unimplemented!()
  }
}