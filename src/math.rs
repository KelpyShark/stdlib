/// KelpyShark Standard Library — Math Module
///
/// Provides mathematical functions: abs, floor, ceil, round, sqrt, pow, min, max, PI, E.

use kelpyshark_interpreter::value::Value;
use super::NativeFn;

pub fn functions() -> Vec<NativeFn> {
    vec![
        ("math_abs", 1, math_abs as fn(Vec<Value>) -> Result<Value, String>),
        ("math_floor", 1, math_floor),
        ("math_ceil", 1, math_ceil),
        ("math_round", 1, math_round),
        ("math_sqrt", 1, math_sqrt),
        ("math_pow", 2, math_pow),
        ("math_min", 2, math_min),
        ("math_max", 2, math_max),
        ("math_pi", 0, math_pi),
        ("math_e", 0, math_e),
    ]
}

fn expect_number(val: &Value, name: &str) -> Result<f64, String> {
    match val {
        Value::Number(n) => Ok(*n),
        other => Err(format!("{}() expected a number, got {}", name, other.type_name())),
    }
}

fn math_abs(args: Vec<Value>) -> Result<Value, String> {
    let n = expect_number(&args[0], "math_abs")?;
    Ok(Value::Number(n.abs()))
}

fn math_floor(args: Vec<Value>) -> Result<Value, String> {
    let n = expect_number(&args[0], "math_floor")?;
    Ok(Value::Number(n.floor()))
}

fn math_ceil(args: Vec<Value>) -> Result<Value, String> {
    let n = expect_number(&args[0], "math_ceil")?;
    Ok(Value::Number(n.ceil()))
}

fn math_round(args: Vec<Value>) -> Result<Value, String> {
    let n = expect_number(&args[0], "math_round")?;
    Ok(Value::Number(n.round()))
}

fn math_sqrt(args: Vec<Value>) -> Result<Value, String> {
    let n = expect_number(&args[0], "math_sqrt")?;
    if n < 0.0 {
        return Err("math_sqrt() cannot take square root of negative number".to_string());
    }
    Ok(Value::Number(n.sqrt()))
}

fn math_pow(args: Vec<Value>) -> Result<Value, String> {
    let base = expect_number(&args[0], "math_pow")?;
    let exp = expect_number(&args[1], "math_pow")?;
    Ok(Value::Number(base.powf(exp)))
}

fn math_min(args: Vec<Value>) -> Result<Value, String> {
    let a = expect_number(&args[0], "math_min")?;
    let b = expect_number(&args[1], "math_min")?;
    Ok(Value::Number(a.min(b)))
}

fn math_max(args: Vec<Value>) -> Result<Value, String> {
    let a = expect_number(&args[0], "math_max")?;
    let b = expect_number(&args[1], "math_max")?;
    Ok(Value::Number(a.max(b)))
}

fn math_pi(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Number(std::f64::consts::PI))
}

fn math_e(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Number(std::f64::consts::E))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(math_abs(vec![Value::Number(-5.0)]).unwrap(), Value::Number(5.0));
        assert_eq!(math_abs(vec![Value::Number(3.0)]).unwrap(), Value::Number(3.0));
    }

    #[test]
    fn test_floor_ceil_round() {
        assert_eq!(math_floor(vec![Value::Number(3.7)]).unwrap(), Value::Number(3.0));
        assert_eq!(math_ceil(vec![Value::Number(3.2)]).unwrap(), Value::Number(4.0));
        assert_eq!(math_round(vec![Value::Number(3.5)]).unwrap(), Value::Number(4.0));
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(math_sqrt(vec![Value::Number(9.0)]).unwrap(), Value::Number(3.0));
        assert!(math_sqrt(vec![Value::Number(-1.0)]).is_err());
    }

    #[test]
    fn test_pow() {
        assert_eq!(math_pow(vec![Value::Number(2.0), Value::Number(3.0)]).unwrap(), Value::Number(8.0));
    }

    #[test]
    fn test_min_max() {
        assert_eq!(math_min(vec![Value::Number(3.0), Value::Number(7.0)]).unwrap(), Value::Number(3.0));
        assert_eq!(math_max(vec![Value::Number(3.0), Value::Number(7.0)]).unwrap(), Value::Number(7.0));
    }

    #[test]
    fn test_pi_e() {
        let pi = math_pi(vec![]).unwrap();
        let e = math_e(vec![]).unwrap();
        match pi { Value::Number(n) => assert!((n - std::f64::consts::PI).abs() < 1e-10), _ => panic!() }
        match e { Value::Number(n) => assert!((n - std::f64::consts::E).abs() < 1e-10), _ => panic!() }
    }

    #[test]
    fn test_type_error() {
        assert!(math_abs(vec![Value::String("hello".to_string())]).is_err());
    }
}
