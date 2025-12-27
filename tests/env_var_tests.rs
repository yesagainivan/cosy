use cosy::{Value, from_str};
use std::env;

#[test]
fn test_env_var_in_string() {
    let key = "COSY_TEST_VAR_STRING";
    unsafe {
        env::set_var(key, "interpolated");
    }

    // Ensure cleanup happens even if test fails (using a defer-like pattern would be better, but simple is fine here)
    // Actually, just let it persist, unique names prevent collision.

    let input = format!(r#"{{ key: "Value is ${{{}}}" }}"#, key);
    let value = from_str(&input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(
            obj.get("key"),
            Some(&Value::String("Value is interpolated".to_string()))
        );
    } else {
        panic!("Expected object");
    }

    unsafe {
        env::remove_var(key);
    }
}

#[test]
fn test_env_var_standalone_integer() {
    let key = "COSY_TEST_VAR_INT";
    unsafe {
        env::set_var(key, "42");
    }

    let input = format!(r#"{{ port: ${{{}}} }}"#, key);
    let value = from_str(&input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("port"), Some(&Value::Integer(42)));
    } else {
        panic!("Expected object");
    }

    unsafe {
        env::remove_var(key);
    }
}

#[test]
fn test_env_var_standalone_bool() {
    let key = "COSY_TEST_VAR_BOOL";
    unsafe {
        env::set_var(key, "true");
    }

    let input = format!(r#"{{ flag: ${{{}}} }}"#, key);
    let value = from_str(&input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("flag"), Some(&Value::Bool(true)));
    } else {
        panic!("Expected object");
    }

    unsafe {
        env::remove_var(key);
    }
}

#[test]
fn test_env_var_standalone_float() {
    let key = "COSY_TEST_VAR_FLOAT";
    unsafe {
        env::set_var(key, "3.14159");
    }

    let input = format!(r#"{{ pi: ${{{}}} }}"#, key);
    let value = from_str(&input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("pi"), Some(&Value::Float(3.14159)));
    } else {
        panic!("Expected object");
    }

    unsafe {
        env::remove_var(key);
    }
}

#[test]
fn test_env_var_standalone_string_fallback() {
    let key = "COSY_TEST_VAR_FALLBACK";
    unsafe {
        env::set_var(key, "not_a_number");
    }

    let input = format!(r#"{{ val: ${{{}}} }}"#, key);
    let value = from_str(&input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(
            obj.get("val"),
            Some(&Value::String("not_a_number".to_string()))
        );
    } else {
        panic!("Expected object");
    }

    unsafe {
        env::remove_var(key);
    }
}

#[test]
fn test_env_var_missing_error() {
    let key = "COSY_TEST_VAR_MISSING_XYZ";
    // Ensure it's unset
    unsafe {
        env::remove_var(key);
    }

    let input = format!(r#"{{ val: ${{{}}} }}"#, key);
    let result = from_str(&input);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[test]
fn test_env_var_escaping() {
    let _input = r#"{ val: "Price: \$${VAR}" }"#;
    // Should parse as literal $${VAR} because the first $ is escaped
    // Wait, my implementation logic:
    // \$ -> literal $
    // So "\$${VAR}" -> literal "$" then literal "${VAR}"?
    // No, logic is:
    // if '\' -> advance, check char. if '$', push '$'.
    // else if '$' && next '{' -> interpolate.
    // So "\$${...}" -> lexer sees '\', consumes it, sees '$', pushes '$'.
    // Next char is '$', then '{'. That matches interpolation start!
    // So it would be: "$<interpolated_value>".

    // To get literal "${VAR}", we need to escape `$` but NOT follow it with `{`?
    // Or rather, if I want literal "${VAR}", I need to escape the `$` that starts the interpolation.
    // My code allows escaping `$`.
    // input: "\${VAR}"
    // lexer: sees '\', next is '$'. Result: push '$'. Advance past '$'.
    // Next is '{'. It's just a char. Pushes '{'.
    // Result: "${VAR}" (literal). Correct.

    let input = r#"{ val: "\${VAR}" }"#;
    let value = from_str(input).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("val"), Some(&Value::String("${VAR}".to_string())));
    } else {
        panic!("Expected object");
    }
}
