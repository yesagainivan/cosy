use crate::syntax::parser;
use crate::value::Value;
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

    match value {
        Value::Object(map) => {
            // Check for "include" key
            // We remove it so it doesn't end up in the final config
            if let Some(Value::String(include_path_str)) = map.shift_remove("include") {
                // 1. Resolve path
                let include_path = base_path.join(&include_path_str);

                // 2. Read and Parse
                let file_content = fs::read_to_string(&include_path)?;
                let mut included_value = parser::from_str(&file_content)?;

                // 3. Recursive resolve on the INCLUDED content (using its own directory as base)
                let new_base = include_path.parent().unwrap_or(Path::new("."));
                resolve_recursive(&mut included_value, new_base, depth + 1)?;

                // 4. Merge
                // The included value MUST be an object to merge into our current object
                if let Value::Object(mut included_map) = included_value {
                    // We merge CURRENT fields INTO included map.
                    // Local fields override included fields.
                    for (k, v) in map.drain(..) {
                        included_map.insert(k, v);
                    }
                    // Replace current map with the merged included map
                    *map = included_map;
                } else {
                    return Err(IncludeError::InvalidIncludeTarget(format!(
                        "Included file '{}' must be an Object, found {}",
                        include_path_str,
                        included_value.type_name()
                    )));
                }
            } else {
                // No include at this level, but check children
                for (_, v) in map.iter_mut() {
                    resolve_recursive(v, base_path, depth)?;
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                resolve_recursive(v, base_path, depth)?;
            }
        }
        _ => {}
    }

    Ok(())
}
