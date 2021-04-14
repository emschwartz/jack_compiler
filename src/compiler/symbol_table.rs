use crate::parser::VarType;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum VarKind {
    Static,
    Field,
    Arg,
    Var,
}

pub struct SymbolEntry {
    pub symbol_type: VarType,
    pub kind: VarKind,
    pub index: usize,
}

pub struct SymbolTable {
    class_symbols: HashMap<String, SymbolEntry>,
    subroutine_symbols: HashMap<String, SymbolEntry>,
    num_statics: usize,
    num_fields: usize,
    num_args: usize,
    num_vars: usize,
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
                self.class_symbols.insert(
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
                self.class_symbols.insert(
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
        }
    }

    pub fn get(&self, name: &str) -> Option<&SymbolEntry> {
        self.subroutine_symbols
            .get(name)
            .or_else(|| self.class_symbols.get(name))
    }

    pub fn get_kind(&self, name: &str) -> Option<VarKind> {
        self.subroutine_symbols
            .get(name)
            .or_else(|| self.class_symbols.get(name))
            .map(|entry| entry.kind)
    }

    pub fn get_type(&self, name: &str) -> Option<&VarType> {
        self.subroutine_symbols
            .get(name)
            .or_else(|| self.class_symbols.get(name))
            .map(|entry| &entry.symbol_type)
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.subroutine_symbols
            .get(name)
            .or_else(|| self.class_symbols.get(name))
            .map(|entry| entry.index)
    }
}
