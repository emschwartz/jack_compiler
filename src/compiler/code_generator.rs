use super::{
    symbol_table::{SymbolTable, VarKind},
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
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            symbol_table: SymbolTable::new(),
            vm_writer: VmWriter::new(),
        }
    }

    pub fn compile_class(&mut self, class: Class) {
        self.symbol_table = SymbolTable::new();

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
            self.compile_subroutine(class.class_name.to_string(), subroutine);
        }
    }

    fn compile_subroutine(&mut self, class_name: String, subroutine: SubroutineDeclaration) {
        self.symbol_table.start_subroutine();

        if subroutine.subroutine_type == SubroutineType::Method {
            unimplemented!();
            // Methods always have 'this' as arg 0
            // self.symbol_table.define(
            //     "this".to_string(),
            //     VarType::ClassName(class_name.to_string()),
            //     VarKind::Arg,
            // );

            // self.vm_writer.write_push(Segment::Constant, index)
        }

        if subroutine.subroutine_type == SubroutineType::Constructor {
            unimplemented!();
        }

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
            &format!("{}.{}", &class_name, subroutine.name),
            subroutine
                .body
                .var_declarations
                .len()
                .try_into()
                .expect("num locals must be a u16"),
        );

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
            Statement::If(statement) => unimplemented!(),
            Statement::While(statement) => unimplemented!(),
            Statement::Let(statement) => unimplemented!(),
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
        let num_args = subroutine_call.expression_list.len();

        for expression in subroutine_call.expression_list {
            self.compile_expression(expression);
        }

        let subroutine_name = if let Some(class_or_var_name) = subroutine_call.class_or_var_name {
            format!("{}.{}", class_or_var_name, subroutine_call.subroutine_name)
        } else {
            subroutine_call.subroutine_name.to_string()
        };
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
                _ => unimplemented!(),
            };
        }
    }

    fn compile_term(&mut self, term: Term) {
        match term {
            Term::Expression(expression) => self.compile_expression(*expression),
            Term::IntegerConstant(int) => self.vm_writer.write_push(Segment::Const, int),
            _ => unimplemented!(),
        }
    }
}
