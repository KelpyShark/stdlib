/// KelpyShark Standard Library — JSON Module
///
/// Provides JSON serialization/deserialization: json_encode, json_decode.

use std::collections::HashMap;
use kelpyshark_interpreter::value::Value;
use super::NativeFn;

pub fn functions() -> Vec<NativeFn> {
    vec![
        ("json_encode", 1, json_encode as fn(Vec<Value>) -> Result<Value, String>),
        ("json_decode", 1, json_decode),
    ]
}

/// Converts a KelpyShark Value into a JSON string.
fn json_encode(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(value_to_json(&args[0])))
}

fn value_to_json(val: &Value) -> String {
    match val {
        Value::Number(n) => {
            if *n == (*n as i64) as f64 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Value::Boolean(b) => format!("{}", b),
        Value::Null => "null".to_string(),
        Value::List(items) => {
            let parts: Vec<String> = items.iter().map(|v| value_to_json(v)).collect();
            format!("[{}]", parts.join(", "))
        }
        Value::Dict(map) => {
            let parts: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, value_to_json(v)))
                .collect();
            format!("{{{}}}", parts.join(", "))
        }
        Value::Function { name, .. } => format!("\"<function {}>\"", name),
        Value::NativeFunction { name, .. } => format!("\"<native {}>\"", name),
        Value::Class { name, .. } => format!("\"<class {}>\"", name),
        Value::Instance { class_name, fields } => {
            let parts: Vec<String> = fields
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, value_to_json(v)))
                .collect();
            format!("{{\"__class__\": \"{}\", {}}}", class_name, parts.join(", "))
        }
    }
}

/// Parses a JSON string into a KelpyShark Value.
/// Supports: numbers, strings, booleans, null, arrays, objects.
fn json_decode(args: Vec<Value>) -> Result<Value, String> {
    let input = match &args[0] {
        Value::String(s) => s.clone(),
        other => return Err(format!("json_decode() expected a string, got {}", other.type_name())),
    };
    let trimmed = input.trim();
    let (val, _) = parse_json_value(trimmed)
        .map_err(|e| format!("json_decode(): {}", e))?;
    Ok(val)
}

fn parse_json_value(input: &str) -> Result<(Value, &str), String> {
    let input = input.trim_start();
    if input.is_empty() {
        return Err("unexpected end of JSON".to_string());
    }

    match input.as_bytes()[0] {
        b'"' => parse_json_string(input),
        b'{' => parse_json_object(input),
        b'[' => parse_json_array(input),
        b't' | b'f' => parse_json_bool(input),
        b'n' => parse_json_null(input),
        b'-' | b'0'..=b'9' => parse_json_number(input),
        c => Err(format!("unexpected character '{}' in JSON", c as char)),
    }
}

fn parse_json_string(input: &str) -> Result<(Value, &str), String> {
    if !input.starts_with('"') {
        return Err("expected '\"'".to_string());
    }
    let rest = &input[1..];
    let mut result = String::new();
    let mut chars = rest.chars();
    let mut consumed = 1; // opening quote

    loop {
        match chars.next() {
            None => return Err("unterminated string".to_string()),
            Some('"') => {
                consumed += 1;
                return Ok((Value::String(result), &input[consumed..]));
            }
            Some('\\') => {
                consumed += 1;
                match chars.next() {
                    Some('n') => { result.push('\n'); consumed += 1; }
                    Some('t') => { result.push('\t'); consumed += 1; }
                    Some('\\') => { result.push('\\'); consumed += 1; }
                    Some('"') => { result.push('"'); consumed += 1; }
                    Some(c) => { result.push(c); consumed += c.len_utf8(); }
                    None => return Err("unterminated escape".to_string()),
                }
            }
            Some(c) => {
                result.push(c);
                consumed += c.len_utf8();
            }
        }
    }
}

fn parse_json_number(input: &str) -> Result<(Value, &str), String> {
    let mut end = 0;
    let bytes = input.as_bytes();
    if end < bytes.len() && bytes[end] == b'-' {
        end += 1;
    }
    while end < bytes.len() && (bytes[end].is_ascii_digit() || bytes[end] == b'.') {
        end += 1;
    }
    // Also handle scientific notation
    if end < bytes.len() && (bytes[end] == b'e' || bytes[end] == b'E') {
        end += 1;
        if end < bytes.len() && (bytes[end] == b'+' || bytes[end] == b'-') {
            end += 1;
        }
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
    }
    let num_str = &input[..end];
    let n: f64 = num_str.parse().map_err(|_| format!("invalid number: {}", num_str))?;
    Ok((Value::Number(n), &input[end..]))
}

fn parse_json_bool(input: &str) -> Result<(Value, &str), String> {
    if input.starts_with("true") {
        Ok((Value::Boolean(true), &input[4..]))
    } else if input.starts_with("false") {
        Ok((Value::Boolean(false), &input[5..]))
    } else {
        Err("expected 'true' or 'false'".to_string())
    }
}

fn parse_json_null(input: &str) -> Result<(Value, &str), String> {
    if input.starts_with("null") {
        Ok((Value::Null, &input[4..]))
    } else {
        Err("expected 'null'".to_string())
    }
}

fn parse_json_array(input: &str) -> Result<(Value, &str), String> {
    let mut rest = &input[1..]; // skip '['
    let mut items = Vec::new();

    rest = rest.trim_start();
    if rest.starts_with(']') {
        return Ok((Value::List(items), &rest[1..]));
    }

    loop {
        let (val, r) = parse_json_value(rest)?;
        items.push(val);
        rest = r.trim_start();
        if rest.starts_with(']') {
            return Ok((Value::List(items), &rest[1..]));
        }
        if rest.starts_with(',') {
            rest = &rest[1..];
        } else {
            return Err("expected ',' or ']' in array".to_string());
        }
    }
}

fn parse_json_object(input: &str) -> Result<(Value, &str), String> {
    let mut rest = &input[1..]; // skip '{'
    let mut map = HashMap::new();

    rest = rest.trim_start();
    if rest.starts_with('}') {
        return Ok((Value::Dict(map), &rest[1..]));
    }

    loop {
        // Parse key (must be a string)
        let (key_val, r) = parse_json_string(rest.trim_start())?;
        let key = match key_val {
            Value::String(s) => s,
            _ => return Err("object key must be a string".to_string()),
        };

        rest = r.trim_start();
        if !rest.starts_with(':') {
            return Err("expected ':' after object key".to_string());
        }
        rest = &rest[1..];

        let (val, r) = parse_json_value(rest)?;
        map.insert(key, val);
        rest = r.trim_start();

        if rest.starts_with('}') {
            return Ok((Value::Dict(map), &rest[1..]));
        }
        if rest.starts_with(',') {
            rest = &rest[1..];
        } else {
            return Err("expected ',' or '}' in object".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(val: &str) -> Value { Value::String(val.to_string()) }

    #[test]
    fn test_encode_number() {
        let result = json_encode(vec![Value::Number(42.0)]).unwrap();
        assert_eq!(result, s("42"));
    }

    #[test]
    fn test_encode_string() {
        let result = json_encode(vec![s("hello")]).unwrap();
        assert_eq!(result, s("\"hello\""));
    }

    #[test]
    fn test_encode_list() {
        let list = Value::List(vec![Value::Number(1.0), Value::Number(2.0)]);
        let result = json_encode(vec![list]).unwrap();
        assert_eq!(result, s("[1, 2]"));
    }

    #[test]
    fn test_encode_dict() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), s("Bob"));
        let result = json_encode(vec![Value::Dict(map)]).unwrap();
        match result {
            Value::String(json) => assert!(json.contains("\"name\": \"Bob\"")),
            _ => panic!(),
        }
    }

    #[test]
    fn test_decode_number() {
        let result = json_decode(vec![s("42")]).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_decode_string() {
        let result = json_decode(vec![s("\"hello\"")]).unwrap();
        assert_eq!(result, s("hello"));
    }

    #[test]
    fn test_decode_bool() {
        assert_eq!(json_decode(vec![s("true")]).unwrap(), Value::Boolean(true));
        assert_eq!(json_decode(vec![s("false")]).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_decode_null() {
        assert_eq!(json_decode(vec![s("null")]).unwrap(), Value::Null);
    }

    #[test]
    fn test_decode_array() {
        let result = json_decode(vec![s("[1, 2, 3]")]).unwrap();
        match result {
            Value::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], Value::Number(1.0));
            }
            _ => panic!("expected list"),
        }
    }

    #[test]
    fn test_decode_object() {
        let result = json_decode(vec![s("{\"name\": \"Bob\", \"age\": 27}")]).unwrap();
        match result {
            Value::Dict(map) => {
                assert_eq!(map.get("name"), Some(&s("Bob")));
                assert_eq!(map.get("age"), Some(&Value::Number(27.0)));
            }
            _ => panic!("expected dict"),
        }
    }

    #[test]
    fn test_roundtrip() {
        let mut map = HashMap::new();
        map.insert("x".to_string(), Value::Number(10.0));
        let original = Value::Dict(map);
        let encoded = json_encode(vec![original.clone()]).unwrap();
        let decoded = json_decode(vec![encoded]).unwrap();
        match decoded {
            Value::Dict(m) => assert_eq!(m.get("x"), Some(&Value::Number(10.0))),
            _ => panic!("expected dict"),
        }
    }
}
