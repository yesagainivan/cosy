use cosy::value::ValueKind;
use cosy::{from_str, to_string};

#[test]
fn test_roundtrip_comments_simple() {
    let input = r#"{
    // comment 1
    a: 1
    // comment 2
    b: 2
    }"#;

    // Parse
    let val = from_str(input).unwrap();

    // Verify comments attached to values
    if let ValueKind::Object(obj) = val.kind {
        let val_a = obj.get("a").unwrap();
        // Check if "comment 1" is attached to "a"'s value
        // Note: comments: ["comment 1", "comment 2"]?
        // "comment 1" is before "a:". In our parser, it's captured before key "a", and attached to value "1".
        // "comment 2" is before "b:". Attached to value "2".

        let val_b = obj.get("b").unwrap();

        // Inspect comments
        println!("Value A comments: {:?}", val_a.comments);
        println!("Value B comments: {:?}", val_b.comments);

        // Note: The formatting of comments in parser might include "// " or just content?
        // Lexer extracts content after `//`.
        // So we expect " comment 1" (with space? Lexer `lex_comment` usually captures remainder of line).
        // Let's assume it captures " comment 1".

        assert!(val_a.comments.iter().any(|c| c.contains("comment 1")));
        assert!(val_b.comments.iter().any(|c| c.contains("comment 2")));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_roundtrip_object_with_comments() {
    let input = r#"{
        // strict mode
        debug: true
        
        // server config
        port: 8080
    }"#;

    let parsed = from_str(input).unwrap();
    let serialized = to_string(&parsed);

    println!("Serialized:\n{}", serialized);

    // Check if comments persist in output
    assert!(serialized.contains("// strict mode"));
    assert!(serialized.contains("// server config"));
}

#[test]
fn test_roundtrip_array_with_comments() {
    let input = r#"[
        // First item
        1,
        // Second item
        2
    ]"#;

    let parsed = from_str(input).unwrap();
    let serialized = to_string(&parsed);

    println!("Serialized array:\n{}", serialized);

    assert!(serialized.contains("// First item"));
    assert!(serialized.contains("// Second item"));
}
