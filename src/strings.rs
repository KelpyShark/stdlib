/// KelpyShark Standard Library — Strings Module
///
/// Provides string manipulation functions: upper, lower, trim, split, join, contains,
/// replace, starts_with, ends_with, char_at, substring.

use kelpyshark_interpreter::value::Value;
use super::NativeFn;

pub fn functions() -> Vec<NativeFn> {
    vec![
        ("str_upper", 1, str_upper as fn(Vec<Value>) -> Result<Value, String>),
        ("str_lower", 1, str_lower),
        ("str_trim", 1, str_trim),
        ("str_split", 2, str_split),
        ("str_join", 2, str_join),
        ("str_contains", 2, str_contains),
        ("str_replace", 3, str_replace),
        ("str_starts_with", 2, str_starts_with),
        ("str_ends_with", 2, str_ends_with),
        ("str_char_at", 2, str_char_at),
        ("str_substring", 3, str_substring),
        ("str_reverse", 1, str_reverse),
    ]
}

fn expect_string(val: &Value, name: &str) -> Result<String, String> {
    match val {
        Value::String(s) => Ok(s.clone()),
        other => Err(format!("{}() expected a string, got {}", name, other.type_name())),
    }
}

fn str_upper(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_upper")?;
    Ok(Value::String(s.to_uppercase()))
}

fn str_lower(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_lower")?;
    Ok(Value::String(s.to_lowercase()))
}

fn str_trim(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_trim")?;
    Ok(Value::String(s.trim().to_string()))
}

fn str_split(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_split")?;
    let delim = expect_string(&args[1], "str_split")?;
    let parts: Vec<Value> = s.split(&delim).map(|p| Value::String(p.to_string())).collect();
    Ok(Value::List(parts))
}

fn str_join(args: Vec<Value>) -> Result<Value, String> {
    let delim = expect_string(&args[0], "str_join")?;
    match &args[1] {
        Value::List(items) => {
            let strs: Result<Vec<String>, String> = items
                .iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s.clone()),
                    other => Ok(format!("{}", other)),
                })
                .collect();
            Ok(Value::String(strs?.join(&delim)))
        }
        other => Err(format!("str_join() expected a list as second argument, got {}", other.type_name())),
    }
}

fn str_contains(args: Vec<Value>) -> Result<Value, String> {
    let haystack = expect_string(&args[0], "str_contains")?;
    let needle = expect_string(&args[1], "str_contains")?;
    Ok(Value::Boolean(haystack.contains(&needle)))
}

fn str_replace(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_replace")?;
    let from = expect_string(&args[1], "str_replace")?;
    let to = expect_string(&args[2], "str_replace")?;
    Ok(Value::String(s.replace(&from, &to)))
}

fn str_starts_with(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_starts_with")?;
    let prefix = expect_string(&args[1], "str_starts_with")?;
    Ok(Value::Boolean(s.starts_with(&prefix)))
}

fn str_ends_with(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_ends_with")?;
    let suffix = expect_string(&args[1], "str_ends_with")?;
    Ok(Value::Boolean(s.ends_with(&suffix)))
}

fn str_char_at(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_char_at")?;
    match &args[1] {
        Value::Number(n) => {
            let idx = *n as usize;
            match s.chars().nth(idx) {
                Some(c) => Ok(Value::String(c.to_string())),
                None => Err(format!("str_char_at(): index {} out of bounds for string of length {}", idx, s.len())),
            }
        }
        other => Err(format!("str_char_at() expected a number as index, got {}", other.type_name())),
    }
}

fn str_substring(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_substring")?;
    let start = match &args[1] {
        Value::Number(n) => *n as usize,
        other => return Err(format!("str_substring() expected a number as start, got {}", other.type_name())),
    };
    let end = match &args[2] {
        Value::Number(n) => *n as usize,
        other => return Err(format!("str_substring() expected a number as end, got {}", other.type_name())),
    };
    let chars: Vec<char> = s.chars().collect();
    if start > chars.len() || end > chars.len() || start > end {
        return Err(format!("str_substring(): invalid range {}..{} for string of length {}", start, end, chars.len()));
    }
    Ok(Value::String(chars[start..end].iter().collect()))
}

fn str_reverse(args: Vec<Value>) -> Result<Value, String> {
    let s = expect_string(&args[0], "str_reverse")?;
    Ok(Value::String(s.chars().rev().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(val: &str) -> Value { Value::String(val.to_string()) }

    #[test]
    fn test_upper_lower() {
        assert_eq!(str_upper(vec![s("hello")]).unwrap(), s("HELLO"));
        assert_eq!(str_lower(vec![s("HELLO")]).unwrap(), s("hello"));
    }

    #[test]
    fn test_trim() {
        assert_eq!(str_trim(vec![s("  hi  ")]).unwrap(), s("hi"));
    }

    #[test]
    fn test_split() {
        let result = str_split(vec![s("a,b,c"), s(",")]).unwrap();
        match result {
            Value::List(items) => assert_eq!(items.len(), 3),
            _ => panic!("expected list"),
        }
    }

    #[test]
    fn test_join() {
        let items = Value::List(vec![s("a"), s("b"), s("c")]);
        assert_eq!(str_join(vec![s("-"), items]).unwrap(), s("a-b-c"));
    }

    #[test]
    fn test_contains() {
        assert_eq!(str_contains(vec![s("hello world"), s("world")]).unwrap(), Value::Boolean(true));
        assert_eq!(str_contains(vec![s("hello"), s("xyz")]).unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_replace() {
        assert_eq!(str_replace(vec![s("hello world"), s("world"), s("Kelpy")]).unwrap(), s("hello Kelpy"));
    }

    #[test]
    fn test_starts_ends_with() {
        assert_eq!(str_starts_with(vec![s("hello"), s("hel")]).unwrap(), Value::Boolean(true));
        assert_eq!(str_ends_with(vec![s("hello"), s("llo")]).unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_char_at() {
        assert_eq!(str_char_at(vec![s("abc"), Value::Number(1.0)]).unwrap(), s("b"));
    }

    #[test]
    fn test_substring() {
        assert_eq!(str_substring(vec![s("hello"), Value::Number(1.0), Value::Number(4.0)]).unwrap(), s("ell"));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(str_reverse(vec![s("abc")]).unwrap(), s("cba"));
    }
}
