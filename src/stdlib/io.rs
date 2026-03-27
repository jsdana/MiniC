use std::io::{self, BufRead};

use crate::interpreter::value::{RuntimeError, Value};

pub fn print_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    let val = args.into_iter().next().unwrap_or(Value::Void);
    println!("{}", val);
    Ok(Value::Void)
}

pub fn read_int_fn(_args: Vec<Value>) -> Result<Value, RuntimeError> {
    let line = read_line()?;
    line.trim()
        .parse::<i64>()
        .map(Value::Int)
        .map_err(|e| RuntimeError::new(format!("readInt: invalid integer input: {}", e)))
}

pub fn read_float_fn(_args: Vec<Value>) -> Result<Value, RuntimeError> {
    let line = read_line()?;
    line.trim()
        .parse::<f64>()
        .map(Value::Float)
        .map_err(|e| RuntimeError::new(format!("readFloat: invalid float input: {}", e)))
}

pub fn read_string_fn(_args: Vec<Value>) -> Result<Value, RuntimeError> {
    let line = read_line()?;
    Ok(Value::Str(line.trim().to_string()))
}

fn read_line() -> Result<String, RuntimeError> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .map_err(|e| RuntimeError::new(format!("IO error: {}", e)))?;
    if line.is_empty() {
        return Err(RuntimeError::new("readString: unexpected EOF"));
    }
    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_fn_integer() {
        // print_fn returns Void without error
        let result = print_fn(vec![Value::Int(42)]);
        assert_eq!(result, Ok(Value::Void));
    }

    #[test]
    fn test_print_fn_bool() {
        let result = print_fn(vec![Value::Bool(true)]);
        assert_eq!(result, Ok(Value::Void));
    }

    #[test]
    fn test_print_fn_array() {
        let result = print_fn(vec![Value::Array(vec![Value::Int(1), Value::Int(2)])]);
        assert_eq!(result, Ok(Value::Void));
    }

    #[test]
    fn test_print_fn_no_args() {
        // falls back to Void without panicking
        let result = print_fn(vec![]);
        assert_eq!(result, Ok(Value::Void));
    }
}
