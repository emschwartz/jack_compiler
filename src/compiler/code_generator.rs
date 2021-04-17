use super::{
    symbol_table::{SymbolEntry, SymbolTable, VarKind, VarType},
    vm_writer::{ArithmeticCommand, Segment, VmWriter},
};
use crate::parser::*;
use std::convert::TryInto;

pub fn compile_class(class: Class) -> impl Iterator<Item = String> {
    let mut code_generator = CodeGenerator::new();
    code_generator.compile_class(class);
    code_generator.vm_writer.into_iter()
}

struct CodeGenerator {
    symbol_table: SymbolTable,
    vm_writer: VmWriter,
    label_count: usize,
    class_name: Option<String>,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            symbol_table: SymbolTable::new(),
            vm_writer: VmWriter::new(),
            label_count: 0,
            class_name: None,
        }
    }

    fn compile_class(&mut self, class: Class) {
        self.symbol_table = SymbolTable::new();
        self.class_name = Some(class.class_name.to_string());

        for var_dec in class.class_var_declarations {
            for name in var_dec.var_names {
                self.symbol_table.define(
                    name,
                    var_dec.var_type.clone(),
                    if var_dec.static_or_field == StaticOrField::Static {
                        VarKind::Static
                    } else {
                        VarKind::Field
                    },
                );
            }
        }

        for subroutine in class.subroutine_declarations {
            self.compile_subroutine(subroutine);
        }
    }

    fn compile_subroutine(&mut self, subroutine: SubroutineDeclaration) {
        self.symbol_table.start_subroutine();
        let class_name = self.class_name.to_owned().unwrap();

        // Add arguments to symbol table
        for (arg_type, arg_name) in subroutine.parameter_list {
            self.symbol_table
                .define(arg_name.to_string(), arg_type.clone(), VarKind::Arg);
        }

        // Add local variables to symbol table
        for var_dec in subroutine.body.var_declarations.iter() {
            for name in var_dec.var_names.iter() {
                self.symbol_table
                    .define(name.to_string(), var_dec.var_type.clone(), VarKind::Var);
            }
        }

        self.vm_writer.write_function(
            &format!("{}.{}", class_name, subroutine.name),
            self.symbol_table
                .var_count(VarKind::Var)
                .try_into()
                .unwrap(),
        );

        // Constructor
        if subroutine.subroutine_type == SubroutineType::Constructor {
            // Allocate memory for the object, then set `this` to the base
            // address of the allocated memory
            let num_fields = self.symbol_table.var_count(VarKind::Field);
            self.vm_writer.write_push(Segment::Const, num_fields);
            self.vm_writer.write_call("Memory.alloc", 1);
            self.vm_writer.write_pop(Segment::Pointer, 0);
        } else if subroutine.subroutine_type == SubroutineType::Method {
            // Set this to arg 0
            self.vm_writer.write_push(Segment::Arg, 0);
            self.vm_writer.write_pop(Segment::Pointer, 0);
        }

        for statement in subroutine.body.statements {
            self.compile_statement(statement);
        }
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Do(statement) => {
                self.compile_subroutine_call(statement.0);
                // Discard return value
                self.vm_writer.write_pop(Segment::Temp, 0);
            }
            Statement::If(statement) => {
                self.label_count += 1;
                let label1 = format!("IF_{}_FALSE", self.label_count);
                let label2 = format!("IF_{}_END", self.label_count);
                self.compile_expression(statement.expression);
                self.vm_writer.write_arithmetic(ArithmeticCommand::Not);
                self.vm_writer.write_if(&label1);
                for statement in statement.if_statements {
                    self.compile_statement(statement);
                }
                self.vm_writer.write_goto(&label2);
                self.vm_writer.write_label(&label1);
                if let Some(else_statements) = statement.else_statements {
                    for statement in else_statements {
                        self.compile_statement(statement);
                    }
                }
                self.vm_writer.write_label(&label2);
            }
            Statement::While(statement) => {
                self.label_count += 1;
                let label1 = format!("WHILE_{}_CONDITION", self.label_count);
                let label2 = format!("WHILE_{}_END", self.label_count);
                self.vm_writer.write_label(&label1);
                self.compile_expression(statement.expression);
                self.vm_writer.write_arithmetic(ArithmeticCommand::Not);
                self.vm_writer.write_if(&label2);
                for statement in statement.statements {
                    self.compile_statement(statement);
                }
                self.vm_writer.write_goto(&label1);
                self.vm_writer.write_label(&label2);
            }
            Statement::Let(statement) => {
                let entry = self
                    .symbol_table
                    .get(&statement.var_name)
                    .expect("Variables must be declared before they are assigned");
                let entry_kind = entry.kind;
                let entry_index = entry.index;

                // If the statement has an array access expression on the left side
                if let Some(expression) = statement.left_side_expression {
                    self.vm_writer
                        .write_push(Segment::from(entry_kind), entry_index);
                    self.compile_expression(expression);
                    self.vm_writer.write_arithmetic(ArithmeticCommand::Add);
                    self.compile_expression(statement.right_side_expression);
                    self.vm_writer.write_pop(Segment::Temp, 0);
                    self.vm_writer.write_pop(Segment::Pointer, 1);
                    self.vm_writer.write_push(Segment::Temp, 0);
                    self.vm_writer.write_pop(Segment::That, 0);
                } else {
                    self.compile_expression(statement.right_side_expression);
                    self.vm_writer
                        .write_pop(Segment::from(entry_kind), entry_index);
                }
            }
            Statement::Return(statement) => {
                if let Some(expression) = statement.0 {
                    self.compile_expression(expression);
                } else {
                    // void functions push a 0 onto the stack before returning
                    self.vm_writer.write_push(Segment::Const, 0);
                }
                self.vm_writer.write_return();
            }
        }
    }

    fn compile_subroutine_call(&mut self, subroutine_call: SubroutineCall) {
        let mut num_args = subroutine_call.expression_list.len();

        // Method call
        if let Some(class_or_var) = &subroutine_call.class_or_var_name {
            if let Some(entry) = self.symbol_table.get(&class_or_var) {
                // The base address of the object is added as arg 0
                num_args += 1;
                self.vm_writer
                    .write_push(Segment::from(entry.kind), entry.index);
            }
        } else {
            // Method in the same class
            num_args += 1;
            self.vm_writer.write_push(Segment::Pointer, 0);
        }

        for expression in subroutine_call.expression_list {
            self.compile_expression(expression);
        }

        let class_name: String = if let Some(class_or_var_name) = subroutine_call.class_or_var_name
        {
            // If we're calling a method on a var, the function we actually need to call
            // is the {class name}.method
            if let Some(SymbolEntry {
                kind: _,
                symbol_type: VarType::ClassName(class_name),
                index: _,
            }) = self.symbol_table.get(&class_or_var_name)
            {
                class_name.to_string()
            } else {
                class_or_var_name
            }
        } else {
            self.class_name.to_owned().unwrap()
        };
        let subroutine_name = format!("{}.{}", class_name, subroutine_call.subroutine_name);
        self.vm_writer
            .write_call(&subroutine_name, num_args.try_into().unwrap());
    }

    fn compile_expression(&mut self, expression: Expression) {
        self.compile_term(expression.term);

        for (op, term) in expression.ops {
            self.compile_term(term);
            match op {
                Op::Plus => self.vm_writer.write_arithmetic(ArithmeticCommand::Add),
                Op::Minus => self.vm_writer.write_arithmetic(ArithmeticCommand::Sub),
                Op::Asterix => self.vm_writer.write_call("Math.multiply", 2),
                Op::GreaterThan => self.vm_writer.write_arithmetic(ArithmeticCommand::Gt),
                Op::LessThan => self.vm_writer.write_arithmetic(ArithmeticCommand::Lt),
                Op::Ampersand => self.vm_writer.write_arithmetic(ArithmeticCommand::And),
                Op::Equals => self.vm_writer.write_arithmetic(ArithmeticCommand::Eq),
                Op::Slash => self.vm_writer.write_call("Math.divide", 2),
                Op::VerticalBar => self.vm_writer.write_arithmetic(ArithmeticCommand::Or),
            };
        }
    }

    fn compile_term(&mut self, term: Term) {
        match term {
            Term::Expression(expression) => self.compile_expression(*expression),
            Term::IntegerConstant(int) => self.vm_writer.write_push(Segment::Const, int),
            Term::KeywordConstant(keyword) => match keyword {
                KeywordConstant::True => {
                    self.vm_writer.write_push(Segment::Const, 1);
                    self.vm_writer.write_arithmetic(ArithmeticCommand::Neg);
                }
                KeywordConstant::False => self.vm_writer.write_push(Segment::Const, 0),
                KeywordConstant::This => {
                    self.vm_writer.write_push(Segment::Pointer, 0);
                }
                KeywordConstant::Null => self.vm_writer.write_push(Segment::Const, 0),
            },
            Term::UnaryOpTerm((op, term)) => {
                self.compile_term(*term);
                match op {
                    UnaryOp::Minus => self.vm_writer.write_arithmetic(ArithmeticCommand::Neg),
                    UnaryOp::Tilde => self.vm_writer.write_arithmetic(ArithmeticCommand::Not),
                };
            }
            Term::VarName(var_name) => {
                let entry = self
                    .symbol_table
                    .get(&&var_name)
                    .expect(&format!("Unknown variable: {}", var_name));
                self.vm_writer
                    .write_push(Segment::from(entry.kind), entry.index);
            }
            Term::SubroutineCall(subroutine_call) => self.compile_subroutine_call(subroutine_call),
            Term::StringConstant(string) => {
                // Create the string
                self.vm_writer.write_push(
                    Segment::Const,
                    string
                        .len()
                        .try_into()
                        .expect("String constant length exceeds u16 size"),
                );
                self.vm_writer.write_call("String.new", 1);
                self.vm_writer.write_pop(Segment::Temp, 0);

                // Append each character
                for c in string.chars() {
                    self.vm_writer.write_push(Segment::Temp, 0);
                    self.vm_writer.write_push(
                        Segment::Const,
                        u32::from(c)
                            .try_into()
                            .expect(&format!("Character {} is outside the range of u16", c)),
                    );
                    // TODO is this the write function signature?
                    self.vm_writer.write_call("String.appendChar", 2);
                }
            }
            Term::VarNameExpression((var_name, expression)) => {
                let entry = self
                    .symbol_table
                    .get(&var_name)
                    .expect(&format!("Unknown variable: {}", var_name));
                self.vm_writer
                    .write_push(Segment::from(entry.kind), entry.index);
                self.compile_expression(*expression);
                self.vm_writer.write_arithmetic(ArithmeticCommand::Add);
                self.vm_writer.write_pop(Segment::Pointer, 0);
                self.vm_writer.write_push(Segment::That, 0);
            }
            _ => {
                println!("{:?}", term);
                unimplemented!()
            }
        }
    }
}
