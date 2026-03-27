use crate::interpreter::value::{RuntimeError, Value};

pub fn pow_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::new(format!(
            "pow expects 2 arguments, got {}",
            args.len()
        )));
    }
    let base = to_float(&args[0], "pow base")?;
    let exp = to_float(&args[1], "pow exponent")?;
    Ok(Value::Float(base.powf(exp)))
}

pub fn sqrt_fn(args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::new(format!(
            "sqrt expects 1 argument, got {}",
            args.len()
        )));
    }
    let x = to_float(&args[0], "sqrt argument")?;
    Ok(Value::Float(x.sqrt()))
}

fn to_float(val: &Value, context: &str) -> Result<f64, RuntimeError> {
    match val {
        Value::Int(n) => Ok(*n as f64),
        Value::Float(x) => Ok(*x),
        v => Err(RuntimeError::new(format!(
            "{}: expected numeric value, got {}",
            context, v
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_int_args() {
        let result = pow_fn(vec![Value::Int(2), Value::Int(10)]);
        assert_eq!(result, Ok(Value::Float(1024.0)));
    }

    #[test]
    fn test_pow_float_args() {
        let result = pow_fn(vec![Value::Float(2.0), Value::Float(0.5)]);
        match result {
            Ok(Value::Float(v)) => assert!((v - 1.4142135).abs() < 1e-5),
            _ => panic!("expected Float"),
        }
    }

    #[test]
    fn test_pow_negative_exponent() {
        let result = pow_fn(vec![Value::Float(2.0), Value::Float(-1.0)]);
        assert_eq!(result, Ok(Value::Float(0.5)));
    }

    #[test]
    fn test_pow_wrong_arity() {
        let result = pow_fn(vec![Value::Float(2.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sqrt_perfect_square() {
        let result = sqrt_fn(vec![Value::Int(4)]);
        assert_eq!(result, Ok(Value::Float(2.0)));
    }

    #[test]
    fn test_sqrt_float() {
        let result = sqrt_fn(vec![Value::Float(2.0)]);
        match result {
            Ok(Value::Float(v)) => assert!((v - 1.4142135).abs() < 1e-5),
            _ => panic!("expected Float"),
        }
    }

    #[test]
    fn test_sqrt_zero() {
        let result = sqrt_fn(vec![Value::Int(0)]);
        assert_eq!(result, Ok(Value::Float(0.0)));
    }

    #[test]
    fn test_sqrt_wrong_type() {
        let result = sqrt_fn(vec![Value::Bool(true)]);
        assert!(result.is_err());
    }
}
