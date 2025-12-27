use indexmap::IndexMap;
use std::fmt;

/// COSY Value type - the core data structure representing any COSY value.
///
/// This enum covers all possible values in COSY:
/// - Null values
/// - Booleans
/// - Integers (distinct from floats)
/// - Floating-point numbers
/// - UTF-8 strings
/// - Arrays of values
/// - Objects (key-value maps with insertion-order preservation)
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// 64-bit signed integer
    Integer(i64),
    /// 64-bit floating-point number
    Float(f64),
    /// UTF-8 string
    String(String),
    /// Homogeneous array of values
    Array(Vec<Value>),
    /// Object (map) with string keys, preserving insertion order
    Object(IndexMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}
