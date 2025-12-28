use crate::value::{Value, ValueKind};

/// Deeply merges `override_val` into `base`.
///
/// Use cases:
/// - **Objects**: Keys in `override_val` replace keys in `base`. Nested objects are merged recursively.
/// - **Arrays**: `override_val` replaces `base`. No array merging (concatenation) is performed.
/// - **Primitives**: `override_val` replaces `base`.
pub fn merge(base: &mut Value, override_val: Value) {
    let Value {
        kind: override_kind,
        comments: override_comments,
    } = override_val;

    // We can only merge if both are objects.
    // However, we can't easily check `kind` without borrowing.
    // But we need to update `*base` if they are NOT both objects.

    let base_is_obj = matches!(base.kind, ValueKind::Object(_));
    let override_is_obj = matches!(override_kind, ValueKind::Object(_));

    if base_is_obj && override_is_obj {
        // Both match, we must destructure both and merge.
        if let ValueKind::Object(base_map) = &mut base.kind {
            if let ValueKind::Object(override_map) = override_kind {
                for (k, v) in override_map {
                    if let Some(base_v) = base_map.get_mut(&k) {
                        merge(base_v, v);
                    } else {
                        base_map.insert(k, v);
                    }
                }
            }
        }
    } else {
        // Just replace
        *base = Value {
            kind: override_kind,
            comments: override_comments,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::parser::from_str;

    #[test]
    fn test_merge_scalars() {
        let mut base = Value::from(ValueKind::Integer(1));
        merge(&mut base, Value::from(ValueKind::Integer(2)));
        assert_eq!(base, Value::from(ValueKind::Integer(2)));
    }

    #[test]
    fn test_merge_different_types() {
        let mut base = Value::from(ValueKind::Integer(1));
        merge(&mut base, Value::from(ValueKind::String("foo".to_string())));
        assert_eq!(base, Value::from(ValueKind::String("foo".to_string())));
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
        if let ValueKind::Object(map) = base.kind {
            assert_eq!(map.get("a"), Some(&Value::from(ValueKind::Integer(1))));
            assert_eq!(map.get("b"), Some(&Value::from(ValueKind::Integer(3)))); // Overridden
            assert_eq!(map.get("c"), Some(&Value::from(ValueKind::Integer(4)))); // Added
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
        if let ValueKind::Object(root) = base.kind {
            if let Some(server) = root.get("server") {
                if let ValueKind::Object(server) = &server.kind {
                    assert_eq!(
                        server.get("host"),
                        Some(&Value::from(ValueKind::String("localhost".to_string())))
                    );
                    assert_eq!(
                        server.get("port"),
                        Some(&Value::from(ValueKind::Integer(9000)))
                    );
                } else {
                    panic!("server not object");
                }
            } else {
                panic!("server not object");
            }
        } else {
            panic!("root not object");
        }
    }
}
