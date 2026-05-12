/// KelpyShark Standard Library — HTTP Module
///
/// Provides basic HTTP client functions for making network requests.
///
/// Functions:
///   http_get(url)             → string response body (or error)
///   http_post(url, body)      → string response body (or error)
///   http_get_status(url)      → number HTTP status code
///   http_post_json(url, json) → string response body with JSON content-type

use kelpyshark_interpreter::value::Value;
use super::NativeFn;

pub fn functions() -> Vec<NativeFn> {
    vec![
        ("http_get",        1, http_get        as fn(Vec<Value>) -> Result<Value, String>),
        ("http_post",       2, http_post),
        ("http_get_status", 1, http_get_status),
        ("http_post_json",  2, http_post_json),
    ]
}

fn http_get(args: Vec<Value>) -> Result<Value, String> {
    let url = extract_string(&args, 0, "http_get")?;
    match ureq::get(&url).call() {
        Ok(resp) => {
            let body = resp.into_string()
                .map_err(|e| format!("http_get(): failed to read response body: {}", e))?;
            Ok(Value::String(body))
        }
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            Err(format!("http_get(): HTTP {} — {}", code, body))
        }
        Err(e) => Err(format!("http_get(): request error: {}", e)),
    }
}

fn http_post(args: Vec<Value>) -> Result<Value, String> {
    let url  = extract_string(&args, 0, "http_post")?;
    let body = extract_string(&args, 1, "http_post")?;
    match ureq::post(&url).set("Content-Type", "text/plain").send_string(&body) {
        Ok(resp) => {
            let text = resp.into_string()
                .map_err(|e| format!("http_post(): failed to read response body: {}", e))?;
            Ok(Value::String(text))
        }
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            Err(format!("http_post(): HTTP {} — {}", code, body))
        }
        Err(e) => Err(format!("http_post(): request error: {}", e)),
    }
}

fn http_get_status(args: Vec<Value>) -> Result<Value, String> {
    let url = extract_string(&args, 0, "http_get_status")?;
    match ureq::get(&url).call() {
        Ok(resp) => Ok(Value::Number(resp.status() as f64)),
        Err(ureq::Error::Status(code, _)) => Ok(Value::Number(code as f64)),
        Err(e) => Err(format!("http_get_status(): request error: {}", e)),
    }
}

fn http_post_json(args: Vec<Value>) -> Result<Value, String> {
    let url  = extract_string(&args, 0, "http_post_json")?;
    let body = extract_string(&args, 1, "http_post_json")?;
    match ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_string(&body)
    {
        Ok(resp) => {
            let text = resp.into_string()
                .map_err(|e| format!("http_post_json(): failed to read response: {}", e))?;
            Ok(Value::String(text))
        }
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            Err(format!("http_post_json(): HTTP {} — {}", code, body))
        }
        Err(e) => Err(format!("http_post_json(): request error: {}", e)),
    }
}

// ── Helper ──────────────────────────────────────────────────

fn extract_string(args: &[Value], idx: usize, fn_name: &str) -> Result<String, String> {
    match args.get(idx) {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(other) => Err(format!(
            "{}(): argument {} must be a string, got {}",
            fn_name, idx + 1, other.type_name()
        )),
        None => Err(format!("{}(): missing argument {}", fn_name, idx + 1)),
    }
}
