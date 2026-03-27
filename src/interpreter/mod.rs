//! Tree-walking interpreter for MiniC.
//!
//! Entry point: `interpret(program: &CheckedProgram) -> Result<(), RuntimeError>`

pub mod eval_expr;
pub mod exec_stmt;
pub mod value;

use crate::environment::Environment;
use crate::ir::ast::CheckedProgram;
use crate::stdlib::NativeRegistry;

use eval_expr::eval_call;
use value::{FnValue, RuntimeError, Value};

/// Interpret a type-checked MiniC program, starting execution at `main`.
pub fn interpret(program: &CheckedProgram) -> Result<(), RuntimeError> {
    let mut env = Environment::<Value>::new();

    // Register native stdlib functions as Value::Fn(FnValue::Native) bindings.
    let registry = NativeRegistry::default();
    for (name, entry) in registry.iter() {
        env.declare(name.clone(), Value::Fn(FnValue::Native(entry.func)));
    }

    // Register user-defined functions as Value::Fn(FnValue::UserDefined) bindings.
    for fun in &program.functions {
        env.declare(fun.name.clone(), Value::Fn(FnValue::UserDefined(fun.clone())));
    }

    if env.get("main").is_none() {
        return Err(RuntimeError::new("no 'main' function found"));
    }

    eval_call("main", vec![], &mut env)?;
    Ok(())
}
