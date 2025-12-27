use crate::value::Value;

/// Deeply merges `override_val` into `base`.
///
/// Use cases:
/// - **Objects**: Keys in `override_val` replace keys in `base`. Nested objects are merged recursively.
/// - **Arrays**: `override_val` replaces `base`. No array merging (concatenation) is performed.
/// - **Primitives**: `override_val` replaces `base`.
pub fn merge(base: &mut Value, override_val: Value) {
    match (base, override_val) {
        (Value::Object(base_map), Value::Object(override_map)) => {
            for (k, v) in override_map {
                if let Some(base_v) = base_map.get_mut(&k) {
                    // Recursive merge for existing keys
                    merge(base_v, v);
                } else {
                    // New key, simply insert
                    base_map.insert(k, v);
                }
            }
        }
        // For all other cases (Array vs Array, Primitive vs Primitive, Mismatched types),
        // we simply replace the base value with the override value.
        (base_val, override_val) => *base_val = override_val,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::parser::from_str;

    #[test]
    fn test_merge_scalars() {
        let mut base = Value::Integer(1);
        merge(&mut base, Value::Integer(2));
        assert_eq!(base, Value::Integer(2));
    }

    #[test]
    fn test_merge_different_types() {
        let mut base = Value::Integer(1);
        merge(&mut base, Value::String("foo".to_string()));
        assert_eq!(base, Value::String("foo".to_string()));
    }

    #[test]
    fn test_merge_arrays_replaces() {
        let mut base = from_str("[1, 2]").unwrap();
        let override_val = from_str("[3]").unwrap();
        merge(&mut base, override_val);
        assert_eq!(base, from_str("[3]").unwrap());
    }

    #[test]
    fn test_merge_objects_simple() {
        let mut base = from_str("{ a: 1, b: 2 }").unwrap();
        let override_val = from_str("{ b: 3, c: 4 }").unwrap();
        merge(&mut base, override_val);

        // Expected: { a: 1, b: 3, c: 4 }
        if let Value::Object(map) = base {
            assert_eq!(map.get("a"), Some(&Value::Integer(1)));
            assert_eq!(map.get("b"), Some(&Value::Integer(3))); // Overridden
            assert_eq!(map.get("c"), Some(&Value::Integer(4))); // Added
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_merge_objects_deep() {
        let mut base = from_str(
            "{ 
            server: { 
                host: \"localhost\", 
                port: 8080 
            } 
        }",
        )
        .unwrap();

        let override_val = from_str(
            "{ 
            server: { 
                port: 9000 
            } 
        }",
        )
        .unwrap();

        merge(&mut base, override_val);

        // Expected: { server: { host: "localhost", port: 9000 } }
        if let Value::Object(root) = base {
            if let Some(Value::Object(server)) = root.get("server") {
                assert_eq!(
                    server.get("host"),
                    Some(&Value::String("localhost".to_string()))
                );
                assert_eq!(server.get("port"), Some(&Value::Integer(9000)));
            } else {
                panic!("server not object");
            }
        } else {
            panic!("root not object");
        }
    }
}
