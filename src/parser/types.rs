use crate::ToXml;
use std::iter;

pub type Identifier = String;

impl ToXml for Identifier {
  fn to_xml(&self) -> String {
      format!("<identifier> {} </identifier>", self)
  }
}

pub struct Class {
  pub class_name: Identifier,
  pub class_var_declarations: Vec<ClassVarDeclaration>,
  pub subroutine_declarations: Vec<SubroutineDeclaration>,
}

impl ToXml for Class {
  fn to_xml(&self) -> String {

    format!("<class>
    <keyword> class </keyword>
    {}
    <symbol> {{ </symbol>
      {}
      {}
    <symbol> }} </symbol>
  </class>", self.class_name.to_xml(),
  self.class_var_declarations.to_xml(),
  self.subroutine_declarations.to_xml())
  }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StaticOrField {
  Static,
  Field
}

impl ToXml for StaticOrField {
  fn to_xml(&self) -> String {
    format!("<keyword> {} </keyword>", if self == &StaticOrField::Static {
      "static"
    } else {
      "field"
    })
  }
}

#[derive(PartialEq, Debug)]
pub enum VarType {
  Int,
  Char,
  Boolean,
  ClassName(Identifier)
}

impl ToXml for VarType {
  fn to_xml(&self) -> String {
    match self {
      VarType::Int => String::from("<keyword> int </keyword>"),
      VarType::Char => String::from("<keyword> char </keyword>"),
      VarType::Boolean => String::from("<keyword> bool </keyword>"),
      VarType::ClassName(class_name) => class_name.to_xml(),
    }
  }
}

pub struct ClassVarDeclaration {
  pub static_or_field: StaticOrField,
  pub var_type: VarType,
  pub var_names: Vec<Identifier>,
}

impl ToXml for ClassVarDeclaration {
  fn to_xml(&self) -> String {
    let mut var_names = intersperse_with(&self.var_names, ", ");
    var_names.push_str("\n<symbol> ; </symbol>\n");

      format!("<classVarDec>
  {}
  {}
  {}
  <symbol> ; </symbol>
</classVarDec>", self.static_or_field.to_xml(),
self.var_type.to_xml(),
var_names)
  }
}

impl ToXml for Vec<ClassVarDeclaration> {
  fn to_xml(&self) -> String {
    intersperse_with(self, "\n")
  }
}

pub enum SubroutineType {
  Constructor,
  Function,
  Method
}

impl ToXml for SubroutineType {
  fn to_xml(&self) -> String {
      match self {
        &SubroutineType::Constructor => String::from("<keyword> constructor </keyword>"),
        &SubroutineType::Function => String::from("<keyword> function </keyword>"),
        &SubroutineType::Method => String::from("<keyword> method </keyword>"),
      }
  }
}

pub type ParameterName = Identifier;
pub type Parameter = (VarType, ParameterName);

impl ToXml for Vec<Parameter> {
  fn to_xml(&self) -> String {
    let parameters = self.iter()
      .map(|p| format!("{}\n{}\n", p.0.to_xml(), p.1.to_xml()))
      .collect::<Vec<String>>();
    let parameters = parameters.join(", ");
    format!("<parameterList>
  {}
</parameterList>", parameters)
  }
}

pub struct SubroutineDeclaration {
  pub subroutine_type: SubroutineType,
  pub return_type: Option<VarType>,
  pub name: Identifier,
  pub parameter_list: Vec<Parameter>,
  pub body: SubroutineBody,
}

impl ToXml for SubroutineDeclaration {
  fn to_xml(&self) -> String {
      format!("<subroutineDec>
  {subroutine_type}
  {return_type}
  {name}
  <symbol> ( </symbol>
  {parameter_list}
  <symbol> ) </symbol>
  {body}
</subroutineDec>",
  subroutine_type = self.subroutine_type.to_xml(),
  return_type = if let Some(ref return_type) = self.return_type {
    return_type.to_xml()
  } else {
    String::from("<keyword> void </keyword>")
  },
  name = self.name.to_xml(),
  parameter_list = self.parameter_list.to_xml(),
  body = self.body.to_xml())
  }
}

impl ToXml for Vec<SubroutineDeclaration> {
  fn to_xml(&self) -> String {
      intersperse_with(&self, "\n")
  }
}

pub struct SubroutineBody {
  pub var_declarations: Vec<VarDeclaration>,
  pub statements: Vec<Statement>,
}

impl ToXml for SubroutineBody {
  fn to_xml(&self) -> String {
      format!("{{
  {}
  {}
}}", self.var_declarations.to_xml(), self.statements.to_xml())
  }
}

pub struct VarDeclaration {
  pub var_type: VarType,
  pub var_names: Vec<Identifier>,
}

impl ToXml for VarDeclaration {
  fn to_xml(&self) -> String {
    format!("<varDec>
  {}
  {}
</varDec>", self.var_type.to_xml(), intersperse_with(&self.var_names, ", "))
  }
}

impl ToXml for Vec<VarDeclaration> {
  fn to_xml(&self) -> String {
    intersperse_with(&self, "\n")
  }
}

pub enum Statement {
  Let(LetStatement),
  If(IfStatement),
  While(WhileStatement),
  Do(DoStatement),
  Return(ReturnStatement)
}

impl ToXml for Statement {
  fn to_xml(&self) -> String {
    match self {
      Statement::Let(s) => s.to_xml(),
      Statement::If(s) => s.to_xml(),
      Statement::While(s) => s.to_xml(),
      Statement::Do(s) => s.to_xml(),
      Statement::Return(s) => s.to_xml(),
    }
  }
}

impl ToXml for Vec<Statement> {
  fn to_xml(&self) -> String {
    iter::once(String::from("<statements>"))
        .chain(self.iter().map(|s| s.to_xml()))
        .chain(iter::once(String::from("</statements>")))
        .collect::<Vec<String>>()
        .join("\n")
  }
}

/** 'let' var_name ('[' left_side_expression ']')? '=' expression ';' */
pub struct LetStatement {
  pub var_name: Identifier,
  pub left_side_expression: Option<Expression>,
  pub right_side_expression: Expression,
}

impl ToXml for LetStatement {
  fn to_xml(&self) -> String {
    let left_side_expression = if let Some(ref expression) = self.left_side_expression {
      format!("[{}]", expression.to_xml())
    } else {
      String::new()
    };
      format!("<letStatement>
  <keyword> let </keyword>
  {}{}
  <symbol> = </symbol>
  {}
  <symbol> ; </symbol>
</letStatement>", self.var_name.to_xml(), left_side_expression, self.right_side_expression.to_xml())
  }
}

pub struct IfStatement {
  pub expression: Expression,
  pub if_statements: Vec<Statement>,
  pub else_statements: Vec<Statement>
}

impl ToXml for IfStatement {
  fn to_xml(&self) -> String {
    let else_statements = if self.else_statements.is_empty() {
      String::new()
    } else {
      format!("<keyword> else </keyword>
<symbol> {{ </symbol>
{}
</symbol> }} </symbol>",
      self.else_statements.to_xml())
    };

    format!("<ifStatement>
  <symbol> ( </symbol>
  {}
  <symbol> ) </symbol>
  <symbol> {{ </symbol>
  {}
  <symbol> }} </symbol>
</ifStatement>
{}", self.expression.to_xml(), self.if_statements.to_xml(), else_statements)
  }
}

pub struct WhileStatement {
  pub expression: Expression,
  pub statements: Vec<Statement>,
}

impl ToXml for WhileStatement {
  fn to_xml(&self) -> String {
      format!("<whileStatement>
  <keyword> while </keyword>
  <symbol> ( </symbol>
  {}
  <symbol> ) </symbol>
  <symbol> {{ </symbol>
  {}
  <symbol> }} </symbol>
</whileStatement>", self.expression.to_xml(), self.statements.to_xml())
  }
}

pub struct DoStatement(pub SubroutineCall);

impl ToXml for DoStatement {
  fn to_xml(&self) -> String {
      unimplemented!()
  }
}

pub struct ReturnStatement(pub Option<Expression>);

impl ToXml for ReturnStatement {
  fn to_xml(&self) -> String {
    format!("<returnStatement>
  <keyword> return </keyword>
  {}<symbol> ; </symbol>
</returnStatement>", if let Some(_expression) = &self.0 {
        unimplemented!()
      } else {
        String::new()
      })
  }
}

pub struct Expression {}

impl ToXml for Expression {
  fn to_xml(&self) -> String {
      unimplemented!()
  }
}

pub struct SubroutineCall {}

fn intersperse_with(vec: &Vec<impl ToXml>, separator: &str) -> String {
  vec.iter().map(|v| v.to_xml()).collect::<Vec<String>>().join(separator)
}