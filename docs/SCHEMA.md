# Cosy Schema Validation Guide

Cosy provides a built-in schema validation system to ensure configuration files match expected structures.

## Basic Syntax

The simplest schema mimics the structure of your data, using strings to define expected types.

```cosy
{
    server: {
        host: "string"
        port: "integer"
        enabled: "boolean"
    }
    // Array with uniform type
    tags: ["string"] 
}
```

### Supported Types
- `"string"`
- `"integer"`
- `"float"`
- `"number"` (matches integer or float)
- `"boolean"` (or `"bool"`)
- `"null"`
- `"any"` (matches anything)

## strict Mode & Unknown Fields

By default, the validator checks for:
1.  **Missing Fields**: All fields in the schema are required by default.
2.  **Type Mismatches**: Values must match their declared type.
3.  **Unknown Fields**: Any field in the config NOT present in the schema is flagged as an error.

## Extended Schema Syntax

For more control, you can use an object definition instead of a simple type string.

### Optional Fields
To make a field optional, use the `optional: true` property.

```cosy
{
    user: {
        name: "string"
        // This field is not required
        nickname: { type: "string", optional: true }
    }
}
```

### Deprecation Warnings
You can mark fields as deprecated to warn users without breaking validation.

```cosy
{
    server: {
        // Did you mean "port"? 
        // Using "port_num" will trigger a warning.
        port_num: { type: "integer", deprecated: "Use 'port' instead" }
        port: "integer"
    }
}
```

## Example Usage (Rust)

```rust
use cosy::{from_str, schema};

let schema_text = r#"{
    name: "string"
    age: { type: "integer", optional: true }
}"#;
let schema = from_str(schema_text).unwrap();

let config_text = r#"{ name: "Alice" }"#; // Valid (age is optional)
let config = from_str(config_text).unwrap();

let report = schema::validate(&config, &schema).unwrap();
// report is empty if valid
```
