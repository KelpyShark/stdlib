/// KelpyShark Standard Library — I/O Module
///
/// Provides file and console I/O functions: read_file, write_file, append_file, input.

use kelpyshark_interpreter::value::Value;
use super::NativeFn;

pub fn functions() -> Vec<NativeFn> {
    vec![
        ("io_read_file", 1, io_read_file as fn(Vec<Value>) -> Result<Value, String>),
        ("io_write_file", 2, io_write_file),
        ("io_append_file", 2, io_append_file),
        ("io_file_exists", 1, io_file_exists),
        ("io_input", 1, io_input),
    ]
}

fn io_read_file(args: Vec<Value>) -> Result<Value, String> {
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        other => return Err(format!("io_read_file() expected a string path, got {}", other.type_name())),
    };
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(format!("io_read_file(): could not read '{}': {}", path, e)),
    }
}

fn io_write_file(args: Vec<Value>) -> Result<Value, String> {
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        other => return Err(format!("io_write_file() expected a string path, got {}", other.type_name())),
    };
    let content = match &args[1] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    match std::fs::write(&path, &content) {
        Ok(_) => Ok(Value::Null),
        Err(e) => Err(format!("io_write_file(): could not write '{}': {}", path, e)),
    }
}

fn io_append_file(args: Vec<Value>) -> Result<Value, String> {
    use std::io::Write;
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        other => return Err(format!("io_append_file() expected a string path, got {}", other.type_name())),
    };
    let content = match &args[1] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("io_append_file(): could not open '{}': {}", path, e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("io_append_file(): write error: {}", e))?;
    Ok(Value::Null)
}

fn io_file_exists(args: Vec<Value>) -> Result<Value, String> {
    let path = match &args[0] {
        Value::String(s) => s.clone(),
        other => return Err(format!("io_file_exists() expected a string path, got {}", other.type_name())),
    };
    Ok(Value::Boolean(std::path::Path::new(&path).exists()))
}

fn io_input(args: Vec<Value>) -> Result<Value, String> {
    let prompt = match &args[0] {
        Value::String(s) => s.clone(),
        other => format!("{}", other),
    };
    use std::io::Write;
    print!("{}", prompt);
    std::io::stdout().flush().map_err(|e| format!("io_input(): {}", e))?;
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .map_err(|e| format!("io_input(): {}", e))?;
    // Strip trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    Ok(Value::String(line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_file() {
        let path = std::env::temp_dir().join("kelpy_test_io.txt");
        let path_str = path.to_string_lossy().to_string();

        // Write
        let result = io_write_file(vec![
            Value::String(path_str.clone()),
            Value::String("hello kelpy".to_string()),
        ]);
        assert!(result.is_ok());

        // Read
        let content = io_read_file(vec![Value::String(path_str.clone())]).unwrap();
        assert_eq!(content, Value::String("hello kelpy".to_string()));

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_append_file() {
        let path = std::env::temp_dir().join("kelpy_test_io_append.txt");
        let path_str = path.to_string_lossy().to_string();

        let _ = std::fs::remove_file(&path);
        io_append_file(vec![Value::String(path_str.clone()), Value::String("line1\n".to_string())]).unwrap();
        io_append_file(vec![Value::String(path_str.clone()), Value::String("line2\n".to_string())]).unwrap();

        let content = io_read_file(vec![Value::String(path_str.clone())]).unwrap();
        assert_eq!(content, Value::String("line1\nline2\n".to_string()));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_file_exists() {
        let result = io_file_exists(vec![Value::String("Cargo.toml".to_string())]).unwrap();
        // May or may not exist depending on CWD, but should not error
        match result {
            Value::Boolean(_) => {}
            _ => panic!("expected boolean"),
        }
    }

    #[test]
    fn test_read_nonexistent() {
        let result = io_read_file(vec![Value::String("/nonexistent_kelpy_file_xyz".to_string())]);
        assert!(result.is_err());
    }
}
