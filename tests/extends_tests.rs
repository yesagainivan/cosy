use cosy::include::resolve;
use cosy::value::ValueKind;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_extends_basic() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Base theme
    fs::write(
        root.join("base.cosy"),
        r#"{
        color: "blue"
        font: "Arial"
    }"#,
    )
    .unwrap();

    // Extending theme
    let input = r#"{
        extends: "base.cosy"
        color: "red"
    }"#;

    let mut val = cosy::from_str(input).unwrap();
    resolve(&mut val, root).unwrap();

    if let ValueKind::Object(map) = val.kind {
        // color should be overridden to red
        assert_eq!(
            map.get("color").unwrap().kind,
            ValueKind::String("red".into())
        );
        // font should be inherited as Arial
        assert_eq!(
            map.get("font").unwrap().kind,
            ValueKind::String("Arial".into())
        );
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_extends_and_include_order() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Base: defines A=1, B=1
    fs::write(root.join("base.cosy"), "{ A: 1, B: 1 }").unwrap();

    // Mixin: defines B=2, C=2
    fs::write(root.join("mixin.cosy"), "{ B: 2, C: 2 }").unwrap();

    // Local: extends Base, includes Mixin, defines C=3
    // Expected Precedence: Local > Mixin > Base
    // So:
    // A: 1 (from Base)
    // B: 2 (Mixin overrides Base)
    // C: 3 (Local overrides Mixin)
    let input = r#"{
        extends: "base.cosy"
        include: "mixin.cosy"
        C: 3
    }"#;

    let mut val = cosy::from_str(input).unwrap();
    resolve(&mut val, root).unwrap();

    if let ValueKind::Object(map) = val.kind {
        assert_eq!(map.get("A").unwrap().kind, ValueKind::Integer(1));
        assert_eq!(map.get("B").unwrap().kind, ValueKind::Integer(2)); // Mixin wins over Base
        assert_eq!(map.get("C").unwrap().kind, ValueKind::Integer(3)); // Local wins over Mixin
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_local_child_recursion_fix() {
    // This reproduces the bug we found: local fields adjacent to `include`/`extends`
    // were not being recursively resolved.
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("base.cosy"), "{ base_val: true }").unwrap();
    fs::write(root.join("nested.cosy"), "{ nested_val: true }").unwrap();

    let input = r#"{
        extends: "base.cosy"
        
        local_child: {
            include: "nested.cosy"
        }
    }"#;

    let mut val = cosy::from_str(input).unwrap();
    resolve(&mut val, root).unwrap();

    if let ValueKind::Object(map) = val.kind {
        assert!(map.contains_key("base_val"));

        let child = map.get("local_child").unwrap();
        if let ValueKind::Object(child_map) = &child.kind {
            // Should contain nested_val, NOT include key
            assert!(child_map.contains_key("nested_val"));
            assert!(!child_map.contains_key("include"));
        } else {
            panic!("local_child should be an object");
        }
    }
}

#[test]
fn test_recursive_extends() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Grandparent
    fs::write(root.join("gp.cosy"), "{ level: 1 }").unwrap();
    // Parent extends Grandparent
    fs::write(
        root.join("parent.cosy"),
        "{ extends: \"gp.cosy\", level: 2 }",
    )
    .unwrap();

    // Child extends Parent
    let input = r#"{
        extends: "parent.cosy"
        level: 3
    }"#;

    let mut val = cosy::from_str(input).unwrap();
    resolve(&mut val, root).unwrap();

    if let ValueKind::Object(map) = val.kind {
        assert_eq!(map.get("level").unwrap().kind, ValueKind::Integer(3));
    }
}
