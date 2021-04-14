use std::iter::IntoIterator;
use std::vec::IntoIter;
pub enum Segment {
    Const,
    Arg,
    Local,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

impl AsRef<str> for Segment {
    fn as_ref(&self) -> &str {
        match self {
            Segment::Const => "constant",
            Segment::Arg => "argument",
            Segment::Local => "local",
            Segment::Static => "static",
            Segment::This => "this",
            Segment::That => "that",
            Segment::Pointer => "pointer",
            Segment::Temp => "temp",
        }
    }
}

pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl AsRef<str> for ArithmeticCommand {
    fn as_ref(&self) -> &str {
        match self {
            ArithmeticCommand::Add => "add",
            ArithmeticCommand::Sub => "sub",
            ArithmeticCommand::Neg => "neg",
            ArithmeticCommand::Eq => "eq",
            ArithmeticCommand::Gt => "gt",
            ArithmeticCommand::Lt => "lt",
            ArithmeticCommand::And => "and",
            ArithmeticCommand::Or => "or",
            ArithmeticCommand::Not => "not",
        }
    }
}
pub struct VmWriter(Vec<String>);

impl VmWriter {
    pub fn new() -> VmWriter {
        VmWriter(Vec::new())
    }

    pub fn write_push(&mut self, segment: Segment, index: u16) {
        self.0.push(format!("push {} {}", segment.as_ref(), index));
    }

    pub fn write_pop(&mut self, segment: Segment, index: u16) {
        self.0.push(format!("pop {} {}", segment.as_ref(), index));
    }

    pub fn write_arithmetic(&mut self, command: ArithmeticCommand) {
        self.0.push(command.as_ref().to_string());
    }

    pub fn write_label(&mut self, label: &str) {
        self.0.push(format!("label {}", label));
    }

    pub fn write_goto(&mut self, label: &str) {
        self.0.push(format!("goto {}", label));
    }

    pub fn write_if(&mut self, label: &str) {
        self.0.push(format!("if-goto {}", label));
    }

    pub fn write_call(&mut self, function_name: &str, num_args: u16) {
        self.0.push(format!("call {} {}", function_name, num_args));
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: u16) {
        self.0
            .push(format!("function {} {}", function_name, num_locals));
    }

    pub fn write_return(&mut self) {
        self.0.push("return".to_string())
    }
}

impl IntoIterator for VmWriter {
    type Item = String;
    type IntoIter = IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
