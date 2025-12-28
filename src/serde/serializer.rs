use crate::value::{Value, ValueKind};
use indexmap::IndexMap;

/// Serialization options for controlling output format
#[derive(Debug, Clone)]
pub struct SerializeOptions {
    /// Number of spaces per indentation level (default: 4)
    pub indent_size: usize,
    /// Use newlines as separators in objects and arrays (default: true)
    pub use_newlines: bool,
    /// Add trailing commas (default: false)
    pub trailing_commas: bool,
}

impl Default for SerializeOptions {
    fn default() -> Self {
        SerializeOptions {
            indent_size: 4,
            use_newlines: true,
            trailing_commas: false,
        }
    }
}

/// Serializer for converting `Value` back to COSY format
pub struct Serializer {
    options: SerializeOptions,
    indent_level: usize,
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer {
    /// Create a new serializer with default options
    pub fn new() -> Self {
        Serializer {
            options: SerializeOptions::default(),
            indent_level: 0,
        }
    }

    /// Create a new serializer with custom options
    pub fn with_options(options: SerializeOptions) -> Self {
        Serializer {
            options,
            indent_level: 0,
        }
    }

    /// Serialize a value to a COSY string
    pub fn serialize(&mut self, value: &Value) -> String {
        self.serialize_value(value)
    }

    fn serialize_value(&mut self, value: &Value) -> String {
        let mut result = String::new();
        // Append comments first
        for comment in &value.comments {
            result.push_str(&self.indent());
            result.push_str("// ");
            result.push_str(comment);
            result.push('\n');
        }

        // Append value
        result.push_str(&self.serialize_value_kind(&value.kind));
        result
    }

    fn serialize_value_kind(&mut self, kind: &ValueKind) -> String {
        match kind {
            ValueKind::Null => "null".to_string(),
            ValueKind::Bool(b) => b.to_string(),
            ValueKind::Integer(i) => i.to_string(),
            ValueKind::Float(f) => {
                // Format floats nicely, avoiding unnecessary decimals
                let s = f.to_string();
                if s.ends_with(".0") { s } else { s }
            }
            ValueKind::String(s) => self.serialize_string(s),
            ValueKind::Array(arr) => self.serialize_array(arr),
            ValueKind::Object(obj) => self.serialize_object(obj),
        }
    }

    fn serialize_string(&self, s: &str) -> String {
        let mut result = String::from("\"");
        for ch in s.chars() {
            match ch {
                '\n' => result.push_str("\\n"),
                '\t' => result.push_str("\\t"),
                '\r' => result.push_str("\\r"),
                '\\' => result.push_str("\\\\"),
                '"' => result.push_str("\\\""),
                _ => result.push(ch),
            }
        }
        result.push('"');
        result
    }

    fn serialize_array(&mut self, arr: &[Value]) -> String {
        if arr.is_empty() {
            return "[]".to_string();
        }

        let mut result = String::from("[");

        if self.options.use_newlines && arr.len() > 1 {
            result.push('\n');
            self.indent_level += 1;

            for (i, item) in arr.iter().enumerate() {
                result.push_str(&self.indent());
                result.push_str(&self.serialize_value(item));

                if i < arr.len() - 1 {
                    result.push(',');
                    result.push('\n');
                } else if self.options.trailing_commas {
                    result.push(',');
                    result.push('\n');
                } else {
                    result.push('\n');
                }
            }

            self.indent_level -= 1;
            result.push_str(&self.indent());
        } else {
            // Single line for short arrays or when use_newlines is false
            for (i, item) in arr.iter().enumerate() {
                result.push_str(&self.serialize_value(item));
                if i < arr.len() - 1 {
                    result.push_str(", ");
                } else if self.options.trailing_commas {
                    result.push(',');
                }
            }
        }

        result.push(']');
        result
    }

    fn serialize_object(&mut self, obj: &IndexMap<String, Value>) -> String {
        if obj.is_empty() {
            return "{}".to_string();
        }

        let mut result = String::from("{");

        if self.options.use_newlines {
            result.push('\n');
            self.indent_level += 1;

            let keys: Vec<_> = obj.keys().collect();
            for (i, key) in keys.iter().enumerate() {
                let value = &obj[*key];

                // Print comments before the key
                for comment in &value.comments {
                    result.push_str(&self.indent());
                    result.push_str("// ");
                    result.push_str(comment);
                    result.push('\n');
                }

                result.push_str(&self.indent());
                result.push_str(key);
                result.push_str(": ");

                result.push_str(&self.serialize_value_kind(&value.kind));

                if i < keys.len() - 1 {
                    result.push(',');
                    result.push('\n');
                } else if self.options.trailing_commas {
                    result.push(',');
                    result.push('\n');
                } else {
                    result.push('\n');
                }
            }

            self.indent_level -= 1;
            result.push_str(&self.indent());
        } else {
            // Single line for compact output
            let keys: Vec<_> = obj.keys().collect();
            for (i, key) in keys.iter().enumerate() {
                let value = &obj[*key];

                for comment in &value.comments {
                    result.push_str("// ");
                    result.push_str(comment);
                    result.push('\n'); // Forced newline for comment
                }

                result.push_str(key);
                result.push_str(": ");
                result.push_str(&self.serialize_value_kind(&value.kind));

                if i < keys.len() - 1 {
                    result.push_str(", ");
                } else if self.options.trailing_commas {
                    result.push(',');
                }
            }
        }

        result.push('}');
        result
    }

    fn indent(&self) -> String {
        " ".repeat(self.indent_level * self.options.indent_size)
    }
}

/// Serialize a value to COSY format with default options
pub fn to_string(value: &Value) -> String {
    let mut serializer = Serializer::new();
    serializer.serialize(value)
}

/// Serialize a value to COSY format with custom options
pub fn to_string_with_options(value: &Value, options: SerializeOptions) -> String {
    let mut serializer = Serializer::with_options(options);
    serializer.serialize(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_null() {
        assert_eq!(to_string(&Value::from(ValueKind::Null)), "null");
    }

    #[test]
    fn test_serialize_booleans() {
        assert_eq!(to_string(&Value::from(ValueKind::Bool(true))), "true");
        assert_eq!(to_string(&Value::from(ValueKind::Bool(false))), "false");
    }

    #[test]
    fn test_serialize_numbers() {
        assert_eq!(to_string(&Value::from(ValueKind::Integer(42))), "42");
        assert_eq!(to_string(&Value::from(ValueKind::Integer(-10))), "-10");
        assert_eq!(to_string(&Value::from(ValueKind::Float(3.14))), "3.14");
    }

    #[test]
    fn test_serialize_strings() {
        assert_eq!(
            to_string(&Value::from(ValueKind::String("hello".to_string()))),
            r#""hello""#
        );
        assert_eq!(
            to_string(&Value::from(ValueKind::String("hello\nworld".to_string()))),
            r#""hello\nworld""#
        );
    }

    #[test]
    fn test_serialize_empty_array() {
        assert_eq!(to_string(&Value::from(ValueKind::Array(vec![]))), "[]");
    }

    #[test]
    fn test_serialize_simple_array() {
        let arr = Value::from(ValueKind::Array(vec![
            Value::from(ValueKind::Integer(1)),
            Value::from(ValueKind::Integer(2)),
            Value::from(ValueKind::Integer(3)),
        ]));
        let output = to_string(&arr);
        // Should use newlines by default
        assert!(output.contains("\n"));
        assert!(output.contains("1"));
        assert!(output.contains("2"));
        assert!(output.contains("3"));
    }

    #[test]
    fn test_serialize_array_compact() {
        let arr = Value::from(ValueKind::Array(vec![
            Value::from(ValueKind::Integer(1)),
            Value::from(ValueKind::Integer(2)),
            Value::from(ValueKind::Integer(3)),
        ]));
        let options = SerializeOptions {
            use_newlines: false,
            ..Default::default()
        };
        let output = to_string_with_options(&arr, options);
        assert_eq!(output, "[1, 2, 3]");
    }

    #[test]
    fn test_serialize_empty_object() {
        let obj = Value::from(ValueKind::Object(IndexMap::new()));
        assert_eq!(to_string(&obj), "{}");
    }

    #[test]
    fn test_serialize_simple_object() {
        let mut obj = IndexMap::new();
        obj.insert(
            "name".to_string(),
            Value::from(ValueKind::String("Alice".to_string())),
        );
        obj.insert("age".to_string(), Value::from(ValueKind::Integer(30)));
        let value = Value::from(ValueKind::Object(obj));

        let output = to_string(&value);
        assert!(output.contains("name"));
        assert!(output.contains("Alice"));
        assert!(output.contains("age"));
        assert!(output.contains("30"));
    }

    #[test]
    fn test_serialize_object_key_order() {
        let mut obj = IndexMap::new();
        obj.insert("first".to_string(), Value::from(ValueKind::Integer(1)));
        obj.insert("second".to_string(), Value::from(ValueKind::Integer(2)));
        obj.insert("third".to_string(), Value::from(ValueKind::Integer(3)));
        let value = Value::from(ValueKind::Object(obj));

        let output = to_string(&value);
        // Keys should appear in insertion order
        let first_pos = output.find("first").expect("missing 'first'");
        let second_pos = output.find("second").expect("missing 'second'");
        let third_pos = output.find("third").expect("missing 'third'");

        assert!(first_pos < second_pos);
        assert!(second_pos < third_pos);
    }

    #[test]
    fn test_serialize_nested_structure() {
        let mut inner = IndexMap::new();
        inner.insert("x".to_string(), Value::from(ValueKind::Integer(1)));
        inner.insert("y".to_string(), Value::from(ValueKind::Integer(2)));

        let mut outer = IndexMap::new();
        outer.insert("point".to_string(), Value::from(ValueKind::Object(inner)));
        let value = Value::from(ValueKind::Object(outer));

        let output = to_string(&value);
        assert!(output.contains("point"));
        assert!(output.contains("x"));
        assert!(output.contains("y"));
    }

    #[test]
    fn test_serialize_with_trailing_commas() {
        let arr = Value::from(ValueKind::Array(vec![
            Value::from(ValueKind::Integer(1)),
            Value::from(ValueKind::Integer(2)),
        ]));
        let options = SerializeOptions {
            trailing_commas: true,
            ..Default::default()
        };
        let output = to_string_with_options(&arr, options);
        assert!(output.contains(",\n")); // trailing comma before closing bracket
    }

    #[test]
    fn test_roundtrip_parse_serialize() {
        use crate::from_str;

        let input = r#"{
            name: "Alice"
            scores: [95, 87, 92]
        }"#;

        let parsed = from_str(input).unwrap();
        let serialized = to_string(&parsed);

        // Should parse again without errors
        let reparsed = from_str(&serialized).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
