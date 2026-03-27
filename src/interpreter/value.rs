use std::fmt;

use crate::ir::ast::CheckedFunDecl;

/// A native (Rust-implemented) MiniC function. Defined here to avoid circular deps with stdlib.
pub type NativeFn = fn(Vec<Value>) -> Result<Value, RuntimeError>;

/// A function value: either a MiniC-defined function or a Rust-native implementation.
#[derive(Clone)]
pub enum FnValue {
    UserDefined(CheckedFunDecl),
    Native(NativeFn),
}

impl PartialEq for FnValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FnValue::UserDefined(a), FnValue::UserDefined(b)) => a == b,
            (FnValue::Native(a), FnValue::Native(b)) => (*a as usize) == (*b as usize),
            _ => false,
        }
    }
}

impl fmt::Debug for FnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FnValue::UserDefined(decl) => write!(f, "UserDefined({})", decl.name),
            FnValue::Native(_) => write!(f, "Native(<fn ptr>)"),
        }
    }
}

/// Runtime value in the MiniC interpreter.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Array(Vec<Value>),
    Void,
    Fn(FnValue),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(x) => write!(f, "{}", x),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Void => write!(f, "void"),
            Value::Array(elems) => {
                write!(f, "[")?;
                for (i, v) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Fn(_) => write!(f, "<function>"),
        }
    }
}

/// A runtime error produced during interpretation.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.message)
    }
}

impl std::error::Error for RuntimeError {}
