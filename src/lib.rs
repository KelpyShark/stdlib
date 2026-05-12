/// KelpyShark Standard Library
///
/// Built-in modules that can be registered into the interpreter.
/// Modules: math, strings, io, json, sys, http

pub mod math;
pub mod strings;
pub mod io;
pub mod json;
pub mod sys;
pub mod http;

use kelpyshark_interpreter::value::Value;

/// A native function definition: (name, arity, function pointer).
pub type NativeFn = (&'static str, usize, fn(Vec<Value>) -> Result<Value, String>);

/// Returns all standard library native functions across all modules.
pub fn all_stdlib_functions() -> Vec<NativeFn> {
    let mut fns = Vec::new();
    fns.extend(math::functions());
    fns.extend(strings::functions());
    fns.extend(io::functions());
    fns.extend(json::functions());
    fns.extend(sys::functions());
    fns.extend(http::functions());
    fns
}
