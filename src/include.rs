use crate::merge;
use crate::syntax::parser;
use crate::value::{Value, ValueKind};
use indexmap::IndexMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

/// Errors that can occur during config inclusion
#[derive(Debug)]
pub enum IncludeError {
    IoError(std::io::Error),
    ParseError(crate::error::CosynError),
    InvalidIncludePath { path: String, message: String },
    RecursionLimitExceeded,
    InvalidIncludeTarget(String),
}

impl fmt::Display for IncludeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IncludeError::IoError(e) => write!(f, "IO error during include: {}", e),
            IncludeError::ParseError(e) => write!(f, "Parse error in included file: {}", e),
            IncludeError::InvalidIncludePath { path, message } => {
                write!(f, "Invalid include path '{}': {}", path, message)
            }
            IncludeError::RecursionLimitExceeded => {
                write!(f, "Recursion limit exceeded (max 10 depth)")
            }
            IncludeError::InvalidIncludeTarget(msg) => write!(f, "Invalid include usage: {}", msg),
        }
    }
}

impl Error for IncludeError {}

impl From<std::io::Error> for IncludeError {
    fn from(err: std::io::Error) -> Self {
        IncludeError::IoError(err)
    }
}

impl From<crate::error::CosynError> for IncludeError {
    fn from(err: crate::error::CosynError) -> Self {
        IncludeError::ParseError(err)
    }
}

/// Recursively resolve "include" keys in a Value.
///
/// If a `Value::Object` contains a key "include" with a string value,
/// that file is loaded, parsed, and merged into the current object.
///
/// - `value`: The configuration value to process (mutable).
/// - `base_path`: The base directory to resolve relative paths against.
pub fn resolve(value: &mut Value, base_path: &Path) -> Result<(), IncludeError> {
    resolve_recursive(value, base_path, 0)
}

const MAX_DEPTH: usize = 10;

fn resolve_recursive(
    value: &mut Value,
    base_path: &Path,
    depth: usize,
) -> Result<(), IncludeError> {
    if depth > MAX_DEPTH {
        return Err(IncludeError::RecursionLimitExceeded);
    }

    match &mut value.kind {
        ValueKind::Object(map) => {
            // 1. Identify and remove directives
            let extends_val = map.shift_remove("extends");
            let include_val = map.shift_remove("include");

            // 2. Resolve local fields (FIX for bug where local includes were ignored)
            for (_, v) in map.iter_mut() {
                resolve_recursive(v, base_path, depth)?;
            }

            // 3. Prepare Base (from `extends`)
            let mut base_config = if let Some(val) = extends_val {
                let path_str = if let ValueKind::String(s) = val.kind {
                    s
                } else {
                    return Err(IncludeError::InvalidIncludeTarget(format!(
                        "Extends value must be a string, found {}",
                        val.type_name()
                    )));
                };
                load_and_resolve(&path_str, base_path, depth)?
            } else {
                Value::object(IndexMap::new())
            };

            // 4. Prepare Mixin (from `include`) and merge into Base
            if let Some(val) = include_val {
                let path_str = if let ValueKind::String(s) = val.kind {
                    s
                } else {
                    return Err(IncludeError::InvalidIncludeTarget(format!(
                        "Include value must be a string, found {}",
                        val.type_name()
                    )));
                };
                let mixin_config = load_and_resolve(&path_str, base_path, depth)?;

                // Merge Mixin INTO Base (Mixin overrides Base)
                // Note: Standard `include` might expect to override `extends`?
                // Yes, extends is deepest base. Include is like a trait/mixin on top.
                merge::merge(&mut base_config, mixin_config);
            }

            // 5. Merge Local (current map) INTO Base (Local overrides Base+Mixin)
            // We take the local map out, wrap it in a Value, merge it into base_config.
            let local_overrides = Value::from(ValueKind::Object(std::mem::take(map)));
            merge::merge(&mut base_config, local_overrides);

            // 6. Put the result back into `value`
            if let ValueKind::Object(merged_map) = base_config.kind {
                *map = merged_map;
            }
        }
        ValueKind::Array(arr) => {
            for v in arr {
                resolve_recursive(v, base_path, depth)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn load_and_resolve(path_str: &str, base_path: &Path, depth: usize) -> Result<Value, IncludeError> {
    let include_path = base_path.join(path_str);
    let file_content = fs::read_to_string(&include_path)?;
    let mut loaded_value = parser::from_str(&file_content)?;

    let new_base = include_path.parent().unwrap_or(Path::new("."));
    resolve_recursive(&mut loaded_value, new_base, depth + 1)?;

    if let ValueKind::Object(_) = loaded_value.kind {
        Ok(loaded_value)
    } else {
        Err(IncludeError::InvalidIncludeTarget(format!(
            "Included/Extended file '{}' must be an Object, found {}",
            path_str,
            loaded_value.type_name()
        )))
    }
}
