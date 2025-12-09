# COSY Format Specification

**COSY** = **C**omfortable **O**bject **S**yntax, **Y**ay!

A human-friendly configuration format designed to be more pleasant to write and read than JSON, while maintaining simplicity and parsability.

---

## Table of Contents

1. [Values](#1-values)
2. [Comments](#2-comments)
3. [Whitespace](#3-whitespace)
4. [Top-Level Values](#4-top-level-values)
5. [Example Document](#5-example-document)
6. [Serde Integration](#6-serde-integration)
7. [Comparison with JSON](#7-comparison-with-json)
8. [Limitations](#8-limitations)
9. [Error Reporting](#9-error-reporting)
10. [Use Cases](#10-use-cases)
11. [Version History](#11-version-history)

---

## 1. Values

COSY supports the following value types:

### 1.1 Null
```cosy
null
```

### 1.2 Booleans
```cosy
true
false
```

### 1.3 Numbers

**Integers** (64-bit signed):
```cosy
42
-10
0
```

**Floats** (64-bit IEEE 754):
```cosy
3.14
-0.5
1e10
2.5e-3
1E+5
```

### 1.4 Strings
Enclosed in double quotes. UTF-8 encoded.

```cosy
"hello"
"multi word string"
"string with \"escaped\" quotes"
```

**Escape sequences:**
- `\n` - newline
- `\t` - tab
- `\r` - carriage return
- `\\` - backslash
- `\"` - double quote

### 1.5 Arrays
Ordered lists of values, enclosed in `[...]`.

```cosy
[1, 2, 3]
["a", "b", "c"]
[1, "mixed", 3.14, true, null]
[[1, 2], [3, 4]]  // nested arrays
```

**Array separators:**
- Commas are the primary separator: `[1, 2, 3]`
- Newlines can replace commas: `[1\n2\n3]`
- Trailing commas are allowed: `[1, 2, 3,]`
- Both can be mixed: `[1, 2\n3,]`

### 1.6 Objects
Key-value maps, enclosed in `{...}`.

```cosy
{name: "Alice", age: 30}
{x: 1, y: 2, z: 3}
```

**Keys:**
- **Unquoted identifiers**: `name:` (alphanumeric + underscore)
- **Quoted strings**: `"my-key":` (allows special characters)

**Key-value separator:** Colon (`:`)

**Object separators:**
- Commas are the primary separator: `{a: 1, b: 2}`
- Newlines can replace commas: `{a: 1\nb: 2}`
- Trailing commas are allowed: `{a: 1, b: 2,}`
- Both can be mixed

---

## 2. Comments

Single-line comments using `//`:

```cosy
// This is a comment
name: "Alice"  // inline comment
// age: 30  // commented out
```

Comments extend to the end of the line and are ignored by the parser.

---

## 3. Whitespace

Whitespace (spaces, tabs) is **ignored** except where it separates tokens.

Newlines have special meaning in arrays and objects—they can act as separators instead of commas (see section 1.5 and 1.6).

---

## 4. Top-Level Values

A COSY document must consist of a single value (which may be an object or array):

```cosy
// Valid: object at root
{name: "Alice"}

// Valid: array at root
[1, 2, 3]

// Valid: scalar at root
42

// Invalid: multiple root values
{a: 1} {b: 2}
```

---

## 5. Example Document

```cosy
// Server configuration
{
    name: "Production Server"
    version: "1.0.0"

    server: {
        host: "0.0.0.0"
        port: 8080
        ssl: true
        cert_path: "/etc/ssl/certs/server.pem"
    }

    database: {
        // Connection pool settings
        url: "postgresql://db.example.com/prod"
        max_connections: 100
        timeout: 30
        retry_attempts: 3
    }

    logging: {
        level: "info"
        format: "json"
        outputs: [
            "stdout"
            "file:/var/log/app.log"
        ]
    }

    features: [
        "auth"
        "api_v2"
        "webhooks"
        "caching"
    ]

    admin_emails: [
        "admin@example.com"
        "ops@example.com"
    ]

    // Feature flags
    debug: false
    maintenance_mode: false
}
```

---

## 6. Serde Integration

COSY has **first-class Serde support** via `cosy::serde_support`, allowing you to deserialize COSY directly into Rust structs and serialize structs back to COSY.

### Basic Usage

```rust
use serde::{Deserialize, Serialize};
use cosy::serde_support;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    port: u16,
    debug: bool,
}

let cosy_text = r#"{
    name: "MyApp"
    port: 8080
    debug: true
}"#;

// Deserialize
let config: Config = serde_support::from_str(cosy_text)?;

// Serialize
let serialized = serde_support::to_string(&config)?;
println!("{}", serialized);
```

### Supported Types

- **Primitives**: `bool`, `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`
- **Strings**: `String`, `&str`
- **Collections**: `Vec<T>`, `HashMap<String, T>` (string keys only)
- **Structs**: Any struct with `#[derive(Serialize, Deserialize)]`
- **Enums**: **Unit variants** (`Active`) and **newtype variants** (`Value(String)`) only
- **Options**: `Option<T>` (None serializes to `null`)

### Enum Support

Only **unit** and **newtype** variants are supported for configuration use cases:

```rust
#[derive(Serialize, Deserialize)]
enum Status {
    Active,      // ✓ Unit variant
    Inactive,    // ✓ Unit variant
    Pending,     // ✓ Unit variant
}

#[derive(Serialize, Deserialize)]
enum Value {
    Number(i32),   // ✓ Newtype variant
    Text(String),  // ✓ Newtype variant
}

// ✗ NOT supported:
// Tuple(i32, String)              - tuple variants
// Struct { a: i32, b: String }    - struct variants
```

### Important Limitations

1. **Enums**: Only unit and newtype variants work. Tuple and struct variants will error during deserialization with a message like "tuple variants not supported; use newtype or unit variants".

2. **Map Keys**: Only `String` keys are supported. Attempting to use non-string keys will fail with "keys must be strings".

3. **Object Key Order**: Key order in objects is **not preserved** (unordered HashMap behavior). Roundtrip serialization will not maintain the original key order.

4. **Comments**: Comments in the original COSY are stripped during parsing. Roundtrip serialization will not preserve comments.

5. **Number Precision**:
   - Integers are stored as `i64` (64-bit signed). Large unsigned integers beyond `i64::MAX` will lose precision.
   - Floats are stored as `f64` (IEEE 754). Values are limited to ~15 significant digits.

6. **Custom Serialization**: Serde's `#[serde(rename)]`, `#[serde(skip)]`, and other attributes are fully supported, allowing fine-grained control over serialization.

---

## 7. Comparison with JSON

| Feature | JSON | COSY |
|---------|------|------|
| Comments | ❌ | ✅ `// comment` |
| Unquoted keys | ❌ | ✅ `name: value` |
| Trailing commas | ❌ | ✅ `[1, 2,]` |
| Newline separators | ❌ | ✅ Can replace commas |
| Integer distinction | ❌ (all numbers) | ✅ `42` vs `3.14` |
| Null support | ✅ | ✅ |
| Strings | ✅ | ✅ |
| Arrays | ✅ | ✅ |
| Objects | ✅ | ✅ |
| Serde support | ⚠️ (custom impl) | ✅ Full integration |

---

## 8. Limitations

### Data Type Limitations

- **No duplicate key handling**: If an object has duplicate keys, the last value wins (standard HashMap behavior).
- **String-only map keys**: Arbitrary key types are not supported; only `String` keys work in Serde integration.
- **Enum variants**: Complex enum variants (tuple, struct) are not supported for configuration use cases.
- **Bytes**: `&[u8]` is serialized as an array of integers `[0, 1, 2, ...]`, not as a string.

### Format Limitations

- **Comments are not preserved**: Roundtrip serialization (parse → serialize) strips all comments.
- **Whitespace normalization**: Original formatting is not preserved; serialization uses standard indentation.
- **Float formatting**: Floats use Rust's default `to_string()` formatting, which may differ from the original input.
- **No custom precision control**: Float output precision cannot be customized per field.

### Serialization Limitations

- **Unordered keys**: Object keys are output in arbitrary order (HashMap iteration order).
- **No format options**: Serialization always uses default formatting (4-space indentation, newlines as separators). Use `crate::to_string_with_options()` to customize.

---

## 9. Error Reporting

The COSY parser provides detailed error messages with **line and column information**:

```
Parse error at line 3, column 15: Expected ':' after object key
```

This helps users quickly locate and fix issues in their configuration files.

### Serde Error Messages

Deserialization errors include the type mismatch details:

```rust
let result: Result<Config, _> = serde_support::from_str("{ port: \"not a number\" }");
// Error: Deserialization error: expected integer
```

---

## 10. Use Cases

COSY is ideal for:
- **Configuration files** (better than JSON, simpler than YAML)
- **Data serialization** (more human-friendly than JSON)
- **Game assets** (fast to write by hand)
- **Build system configs** (comments for documentation)
- **Settings files** (with Serde integration for type-safe deserialization)

**COSY is NOT recommended for:**
- Complex type serialization (use JSON or bincode)
- Preserving comments (original comments are lost during roundtrip)
- Non-string map keys
- Complex enum variants

---

## 11. ABNF Grammar (Informal)

```
document = value

value = null | boolean | number | string | array | object

null = "null"
boolean = "true" | "false"

number = integer | float
integer = ["-"] digit+
float = ["-"] digit+ "." digit+ | digit+ ["."] digit+ ("e"|"E") ["+"|"-"] digit+

string = '"' (char | escape)* '"'
escape = "\" ("n"|"t"|"r"|"\"|'"')

array = "[" [value (separator value)*] "]"
object = "{" [pair (separator pair)*] "}"

pair = key ":" value
key = identifier | string
identifier = (letter | "_") (letter | digit | "_")*

separator = "," | newline | ("," newline) | (newline ",")
```

---

## 12. Version History

- **1.0.0** (Current) - Initial specification with full Serde support and comprehensive error handling
