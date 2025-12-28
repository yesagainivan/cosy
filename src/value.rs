use indexmap::IndexMap;
use std::fmt;

/// COSY Value type - the core data structure representing any COSY value.
#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub kind: ValueKind,
    pub comments: Vec<String>,
}

impl Value {
    pub fn new(kind: ValueKind) -> Self {
        Value {
            kind,
            comments: Vec::new(),
        }
    }

    pub fn with_comments(kind: ValueKind, comments: Vec<String>) -> Self {
        Value { kind, comments }
    }

    /// Get the string representation of the value's type
    pub fn type_name(&self) -> &'static str {
        self.kind.type_name()
    }

    // Helper constructors
    pub fn null() -> Self {
        Self::new(ValueKind::Null)
    }
    pub fn boolean(b: bool) -> Self {
        Self::new(ValueKind::Bool(b))
    }
    pub fn integer(i: i64) -> Self {
        Self::new(ValueKind::Integer(i))
    }
    pub fn float(f: f64) -> Self {
        Self::new(ValueKind::Float(f))
    }
    pub fn string(s: String) -> Self {
        Self::new(ValueKind::String(s))
    }
    pub fn array(arr: Vec<Value>) -> Self {
        Self::new(ValueKind::Array(arr))
    }
    pub fn object(obj: IndexMap<String, Value>) -> Self {
        Self::new(ValueKind::Object(obj))
    }
}

/// The actual data variant of a COSY value
#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
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

impl ValueKind {
    pub fn type_name(&self) -> &'static str {
        match self {
            ValueKind::Null => "null",
            ValueKind::Bool(_) => "boolean",
            ValueKind::Integer(_) => "integer",
            ValueKind::Float(_) => "float",
            ValueKind::String(_) => "string",
            ValueKind::Array(_) => "array",
            ValueKind::Object(_) => "object",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // For now, simple display doesn't show comments to keep debug output clean
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueKind::Null => write!(f, "null"),
            ValueKind::Bool(b) => write!(f, "{}", b),
            ValueKind::Integer(i) => write!(f, "{}", i),
            ValueKind::Float(fl) => write!(f, "{}", fl),
            ValueKind::String(s) => write!(f, "\"{}\"", s),
            ValueKind::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            ValueKind::Object(obj) => {
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

// Implement From conversions for convenience
impl From<ValueKind> for Value {
    fn from(kind: ValueKind) -> Self {
        Self::new(kind)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::boolean(v)
    }
}
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::integer(v)
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::float(v)
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::string(v)
    }
}
impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::string(v.to_string())
    }
}
