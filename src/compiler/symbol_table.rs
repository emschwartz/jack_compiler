pub use crate::parser::VarType;
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarKind {
    Static,
    Field,
    Arg,
    Var,
}

#[derive(Debug)]
pub struct SymbolEntry {
    pub symbol_type: VarType,
    pub kind: VarKind,
    pub index: u16,
}

#[derive(Debug)]
pub struct SymbolTable {
    class_symbols: HashMap<String, SymbolEntry>,
    subroutine_symbols: HashMap<String, SymbolEntry>,
    num_statics: u16,
    num_fields: u16,
    num_args: u16,
    num_vars: u16,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            class_symbols: HashMap::new(),
            subroutine_symbols: HashMap::new(),
            num_statics: 0,
            num_fields: 0,
            num_args: 0,
            num_vars: 0,
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_symbols.clear();
        self.num_args = 0;
        self.num_vars = 0;
    }

    pub fn define(&mut self, name: String, symbol_type: VarType, kind: VarKind) {
        match kind {
            VarKind::Static => {
                self.class_symbols.insert(
                    name,
                    SymbolEntry {
                        symbol_type,
                        kind,
                        index: self.num_statics,
                    },
                );
                self.num_statics += 1;
            }
            VarKind::Field => {
                self.class_symbols.insert(
                    name,
                    SymbolEntry {
                        symbol_type,
                        kind,
                        index: self.num_fields,
                    },
                );
                self.num_fields += 1;
            }
            VarKind::Arg => {
                self.subroutine_symbols.insert(
                    name,
                    SymbolEntry {
                        symbol_type,
                        kind,
                        index: self.num_args,
                    },
                );
                self.num_args += 1;
            }
            VarKind::Var => {
                self.subroutine_symbols.insert(
                    name,
                    SymbolEntry {
                        symbol_type,
                        kind,
                        index: self.num_vars,
                    },
                );
                self.num_vars += 1;
            }
        }
    }

    pub fn var_count(&self, kind: VarKind) -> usize {
        match kind {
            VarKind::Static => self.num_statics,
            VarKind::Field => self.num_fields,
            VarKind::Arg => self.num_args,
            VarKind::Var => self.num_vars,
        }.try_into().expect("Var count exceeds u16 max")
    }

    pub fn get(&self, name: &str) -> Option<&SymbolEntry> {
        self.subroutine_symbols
            .get(name)
            .or_else(|| self.class_symbols.get(name))
    }
}
