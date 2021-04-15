use crate::ToXml;
use std::iter;

pub type Identifier = String;

impl ToXml for Identifier {
    fn to_xml(&self) -> String {
        format!("<identifier> {} </identifier>", self)
    }
}

#[derive(Debug)]
pub struct Class {
    pub class_name: Identifier,
    pub class_var_declarations: Vec<ClassVarDeclaration>,
    pub subroutine_declarations: Vec<SubroutineDeclaration>,
}

impl ToXml for Class {
    fn to_xml(&self) -> String {
        format!(
            "<class>
    <keyword> class </keyword>
    {}
    <symbol> {{ </symbol>
      {}
      {}
    <symbol> }} </symbol>
  </class>",
            self.class_name.to_xml(),
            self.class_var_declarations.to_xml(),
            self.subroutine_declarations.to_xml()
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum StaticOrField {
    Static,
    Field,
}

impl ToXml for StaticOrField {
    fn to_xml(&self) -> String {
        format!(
            "<keyword> {} </keyword>",
            if self == &StaticOrField::Static {
                "static"
            } else {
                "field"
            }
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum VarType {
    Int,
    Char,
    Boolean,
    ClassName(Identifier),
}

impl ToXml for VarType {
    fn to_xml(&self) -> String {
        match self {
            VarType::Int => String::from("<keyword> int </keyword>"),
            VarType::Char => String::from("<keyword> char </keyword>"),
            VarType::Boolean => String::from("<keyword> boolean </keyword>"),
            VarType::ClassName(class_name) => class_name.to_xml(),
        }
    }
}

#[derive(Debug)]
pub struct ClassVarDeclaration {
    pub static_or_field: StaticOrField,
    pub var_type: VarType,
    pub var_names: Vec<Identifier>,
}

impl ToXml for ClassVarDeclaration {
    fn to_xml(&self) -> String {
        format!(
            "<classVarDec>
  {}
  {}
  {}
  <symbol> ; </symbol>
</classVarDec>",
            self.static_or_field.to_xml(),
            self.var_type.to_xml(),
            intersperse_with(&self.var_names, "\n<symbol> , </symbol>\n"),
        )
    }
}

impl ToXml for Vec<ClassVarDeclaration> {
    fn to_xml(&self) -> String {
        intersperse_with(self, "\n")
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
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
        let parameters = self
            .iter()
            .map(|p| format!("{}\n{}\n", p.0.to_xml(), p.1.to_xml()))
            .collect::<Vec<String>>();
        let parameters = parameters.join("\n<symbol> , </symbol>\n");
        format!(
            "<parameterList>
  {}
</parameterList>",
            parameters
        )
    }
}

#[derive(Debug)]
pub struct SubroutineDeclaration {
    pub subroutine_type: SubroutineType,
    pub return_type: Option<VarType>,
    pub name: Identifier,
    pub parameter_list: Vec<Parameter>,
    pub body: SubroutineBody,
}

impl ToXml for SubroutineDeclaration {
    fn to_xml(&self) -> String {
        format!(
            "<subroutineDec>
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
            body = self.body.to_xml()
        )
    }
}

impl ToXml for Vec<SubroutineDeclaration> {
    fn to_xml(&self) -> String {
        intersperse_with(&self, "\n")
    }
}

#[derive(Debug)]
pub struct SubroutineBody {
    pub var_declarations: Vec<VarDeclaration>,
    pub statements: Vec<Statement>,
}

impl ToXml for SubroutineBody {
    fn to_xml(&self) -> String {
        format!(
            "<subroutineBody>
<symbol> {{ </symbol>
{}
{}
<symbol> }} </symbol>
</subroutineBody>",
            self.var_declarations.to_xml(),
            self.statements.to_xml()
        )
    }
}

#[derive(Debug)]
pub struct VarDeclaration {
    pub var_type: VarType,
    pub var_names: Vec<Identifier>,
}

impl ToXml for VarDeclaration {
    fn to_xml(&self) -> String {
        format!(
            "<varDec>
<keyword> var </keyword>
  {}
  {}
<symbol> ; </symbol>
</varDec>
",
            self.var_type.to_xml(),
            intersperse_with(&self.var_names, "\n<symbol> , </symbol>\n")
        )
    }
}

impl ToXml for Vec<VarDeclaration> {
    fn to_xml(&self) -> String {
        self.iter()
            .map(|v| v.to_xml())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    While(WhileStatement),
    Do(DoStatement),
    Return(ReturnStatement),
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
#[derive(Debug)]
pub struct LetStatement {
    pub var_name: Identifier,
    pub left_side_expression: Option<Expression>,
    pub right_side_expression: Expression,
}

impl ToXml for LetStatement {
    fn to_xml(&self) -> String {
        let left_side_expression = if let Some(ref expression) = self.left_side_expression {
            format!(
                "<symbol> [ </symbol>
{}
<symbol> ] </symbol>",
                expression.to_xml()
            )
        } else {
            String::new()
        };
        format!(
            "<letStatement>
  <keyword> let </keyword>
  {}
  {}
  <symbol> = </symbol>
  {}
  <symbol> ; </symbol>
</letStatement>",
            self.var_name.to_xml(),
            left_side_expression,
            self.right_side_expression.to_xml()
        )
    }
}

#[derive(Debug)]
pub struct IfStatement {
    pub expression: Expression,
    pub if_statements: Vec<Statement>,
    pub else_statements: Option<Vec<Statement>>,
}

impl ToXml for IfStatement {
    fn to_xml(&self) -> String {
        let else_statements = if let Some(else_statements) = &self.else_statements {
            format!(
                "<keyword> else </keyword>
<symbol> {{ </symbol>
{}
<symbol> }} </symbol>
",
                else_statements.to_xml()
            )
        } else {
            String::new()
        };

        format!(
            "<ifStatement>
<keyword> if </keyword>
  <symbol> ( </symbol>
  {}
  <symbol> ) </symbol>
  <symbol> {{ </symbol>
  {}
  <symbol> }} </symbol>
{}
</ifStatement>",
            self.expression.to_xml(),
            if self.if_statements.is_empty() {
                String::new()
            } else {
                self.if_statements.to_xml()
            },
            else_statements
        )
    }
}

#[derive(Debug)]
pub struct WhileStatement {
    pub expression: Expression,
    pub statements: Vec<Statement>,
}

impl ToXml for WhileStatement {
    fn to_xml(&self) -> String {
        format!(
            "<whileStatement>
  <keyword> while </keyword>
  <symbol> ( </symbol>
  {}
  <symbol> ) </symbol>
  <symbol> {{ </symbol>
  {}
  <symbol> }} </symbol>
</whileStatement>",
            self.expression.to_xml(),
            self.statements.to_xml()
        )
    }
}

#[derive(Debug)]
pub struct DoStatement(pub SubroutineCall);

impl ToXml for DoStatement {
    fn to_xml(&self) -> String {
        format!(
            "<doStatement>
<keyword> do </keyword>
{}
<symbol> ; </symbol>
</doStatement>",
            self.0.to_xml()
        )
    }
}

#[derive(Debug)]
pub struct ReturnStatement(pub Option<Expression>);

impl ToXml for ReturnStatement {
    fn to_xml(&self) -> String {
        format!(
            "<returnStatement>
  <keyword> return </keyword>
  {}
  <symbol> ; </symbol>
</returnStatement>",
            if let Some(expression) = &self.0 {
                format!("{}\n", expression.to_xml())
            } else {
                String::new()
            }
        )
    }
}

#[derive(Debug)]
pub struct Expression {
    pub term: Term,
    pub ops: Vec<(Op, Term)>,
}

impl ToXml for Expression {
    fn to_xml(&self) -> String {
        format!(
            "<expression>
{}
{}
</expression>",
            self.term.to_xml(),
            self.ops
                .iter()
                .flat_map(|(op, term)| vec![op.to_xml(), term.to_xml()].into_iter())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug)]
pub enum Term {
    IntegerConstant(u16),
    StringConstant(String),
    KeywordConstant(KeywordConstant),
    VarName(Identifier),
    VarNameExpression((Identifier, Box<Expression>)),
    SubroutineCall(SubroutineCall),
    Expression(Box<Expression>),
    UnaryOpTerm((UnaryOp, Box<Term>)),
}

impl ToXml for Term {
    fn to_xml(&self) -> String {
        let inner = match &self {
            &Term::IntegerConstant(int) => format!("<integerConstant> {} </integerConstant>", int),
            &Term::StringConstant(string) => {
                format!("<stringConstant> {} </stringConstant>", string)
            }
            &Term::KeywordConstant(keyword) => keyword.to_xml(),
            &Term::VarName(var_name) => var_name.to_xml(),
            &Term::VarNameExpression((var_name, expression)) => format!(
                "{}
<symbol> [ </symbol>
{}
<symbol> ] </symbol>",
                var_name.to_xml(),
                expression.to_xml()
            ),
            &Term::SubroutineCall(subroutine_call) => subroutine_call.to_xml(),
            &Term::Expression(expression) => format!(
                "<symbol> ( </symbol>
{}
<symbol> ) </symbol>",
                expression.to_xml()
            ),
            &Term::UnaryOpTerm((op, term)) => format!("{}\n{}", op.to_xml(), term.to_xml()),
        };
        format!("<term>\n{}\n</term>", inner)
    }
}

#[derive(Debug)]
pub struct SubroutineCall {
    pub class_or_var_name: Option<Identifier>,
    pub subroutine_name: Identifier,
    pub expression_list: Vec<Expression>,
}

impl ToXml for SubroutineCall {
    fn to_xml(&self) -> String {
        let mut string = if let Some(class_or_var_name) = &self.class_or_var_name {
            format!(
                "{}
<symbol> . </symbol>
",
                class_or_var_name.to_xml()
            )
        } else {
            String::new()
        };
        string.push_str(&format!(
            "{}
<symbol> ( </symbol>
<expressionList>
{}
</expressionList>
<symbol> ) </symbol>",
            self.subroutine_name.to_xml(),
            self.expression_list
                .iter()
                .map(|e| e.to_xml())
                .collect::<Vec<String>>()
                .join("\n<symbol> , </symbol>\n")
        ));
        string
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Asterix,
    Slash,
    Ampersand,
    VerticalBar,
    LessThan,
    GreaterThan,
    Equals,
}

impl AsRef<str> for Op {
    fn as_ref(&self) -> &str {
        match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Asterix => "*",
            Op::Slash => "/",
            Op::Ampersand => "&",
            Op::VerticalBar => "|",
            Op::LessThan => "<",
            Op::GreaterThan => ">",
            Op::Equals => "=",
        }
    }
}

impl ToXml for Op {
    fn to_xml(&self) -> String {
        match self {
            Op::LessThan => String::from("<symbol> &lt; </symbol>"),
            Op::GreaterThan => String::from("<symbol> &gt; </symbol>"),
            Op::Ampersand => String::from("<symbol> &amp; </symbol>"),
            _ => format!("<symbol> {} </symbol>", self.as_ref()),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus,
    Tilde,
}

impl AsRef<str> for UnaryOp {
    fn as_ref(&self) -> &str {
        match self {
            UnaryOp::Minus => "-",
            UnaryOp::Tilde => "~",
        }
    }
}

impl ToXml for UnaryOp {
    fn to_xml(&self) -> String {
        format!("<symbol> {} </symbol>", self.as_ref())
    }
}

#[derive(Debug)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

impl AsRef<str> for KeywordConstant {
    fn as_ref(&self) -> &str {
        match self {
            &KeywordConstant::True => "true",
            &KeywordConstant::False => "false",
            &KeywordConstant::Null => "null",
            &KeywordConstant::This => "this",
        }
    }
}

impl ToXml for KeywordConstant {
    fn to_xml(&self) -> String {
        format!("<keyword> {} </keyword>", self.as_ref())
    }
}

fn intersperse_with(vec: &Vec<impl ToXml>, separator: &str) -> String {
    vec.iter()
        .map(|v| v.to_xml())
        .collect::<Vec<String>>()
        .join(separator)
}
