/// KelpyShark Standard Library — System Module
///
/// Provides system-level functions: env, exit, clock, args, cwd, platform.

use kelpyshark_interpreter::value::Value;
use super::NativeFn;

pub fn functions() -> Vec<NativeFn> {
    vec![
        ("sys_env", 1, sys_env as fn(Vec<Value>) -> Result<Value, String>),
        ("sys_exit", 1, sys_exit),
        ("sys_clock", 0, sys_clock),
        ("sys_args", 0, sys_args),
        ("sys_cwd", 0, sys_cwd),
        ("sys_platform", 0, sys_platform),
        ("sys_sleep", 1, sys_sleep),
    ]
}

fn sys_env(args: Vec<Value>) -> Result<Value, String> {
    let key = match &args[0] {
        Value::String(s) => s.clone(),
        other => return Err(format!("sys_env() expected a string key, got {}", other.type_name())),
    };
    match std::env::var(&key) {
        Ok(val) => Ok(Value::String(val)),
        Err(_) => Ok(Value::Null),
    }
}

fn sys_exit(args: Vec<Value>) -> Result<Value, String> {
    let code = match &args[0] {
        Value::Number(n) => *n as i32,
        _ => 0,
    };
    std::process::exit(code);
}

fn sys_clock(_args: Vec<Value>) -> Result<Value, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("sys_clock(): {}", e))?;
    Ok(Value::Number(duration.as_secs_f64()))
}

fn sys_args(_args: Vec<Value>) -> Result<Value, String> {
    let args: Vec<Value> = std::env::args()
        .map(|a| Value::String(a))
        .collect();
    Ok(Value::List(args))
}

fn sys_cwd(_args: Vec<Value>) -> Result<Value, String> {
    let cwd = std::env::current_dir()
        .map_err(|e| format!("sys_cwd(): {}", e))?;
    Ok(Value::String(cwd.to_string_lossy().to_string()))
}

fn sys_platform(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(std::env::consts::OS.to_string()))
}

fn sys_sleep(args: Vec<Value>) -> Result<Value, String> {
    let seconds = match &args[0] {
        Value::Number(n) => *n,
        other => return Err(format!("sys_sleep() expected a number, got {}", other.type_name())),
    };
    if seconds < 0.0 {
        return Err("sys_sleep(): cannot sleep for negative duration".to_string());
    }
    std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
    Ok(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform() {
        let result = sys_platform(vec![]).unwrap();
        match result {
            Value::String(s) => assert!(!s.is_empty()),
            _ => panic!("expected string"),
        }
    }

    #[test]
    fn test_cwd() {
        let result = sys_cwd(vec![]).unwrap();
        match result {
            Value::String(s) => assert!(!s.is_empty()),
            _ => panic!("expected string"),
        }
    }

    #[test]
    fn test_clock() {
        let result = sys_clock(vec![]).unwrap();
        match result {
            Value::Number(n) => assert!(n > 0.0),
            _ => panic!("expected number"),
        }
    }

    #[test]
    fn test_args() {
        let result = sys_args(vec![]).unwrap();
        match result {
            Value::List(_) => {}
            _ => panic!("expected list"),
        }
    }

    #[test]
    fn test_env_existing() {
        // PATH should exist on all platforms
        let result = sys_env(vec![Value::String("PATH".to_string())]).unwrap();
        match result {
            Value::String(s) => assert!(!s.is_empty()),
            Value::Null => {} // might be null in some env
            _ => panic!("expected string or null"),
        }
    }

    #[test]
    fn test_env_nonexistent() {
        let result = sys_env(vec![Value::String("KELPY_NONEXISTENT_VAR_XYZ".to_string())]).unwrap();
        assert_eq!(result, Value::Null);
    }
}
