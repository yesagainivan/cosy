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
8. [Use Cases & Design Philosophy](#8-use-cases--design-philosophy)
9. [Error Reporting](#9-error-reporting)
10. [Production Readiness](#10-production-readiness)
11. [Version History](#11-version-history)
12. [Future Roadmap](#12-future-roadmap)

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
use cosy::serde;

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
let config: Config = serde::from_str(cosy_text)?;

// Serialize
let serialized = serde::to_string(&config)?;
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

### Key Order Preservation

**COSY now preserves object key order!** Objects maintain insertion-order semantics, so your config keys will appear in the same order they were defined:

```cosy
{
    version: "1.0.0"      // First
    name: "MyApp"         // Second
    debug: true           // Third
}
```

When serialized and deserialized, this order is maintained. This makes COSY ideal for configuration files where logical grouping matters.

### Important Design Choices

1. **Enums**: Only unit and newtype variants work. Tuple and struct variants will error during deserialization with a message like "tuple variants not supported; use newtype or unit variants".

2. **Map Keys**: Only `String` keys are supported. Attempting to use non-string keys will fail with "keys must be strings".

3. **Comments**: Comments in the original COSY are stripped during parsing. Roundtrip serialization will not preserve comments.

4. **Number Precision**:
   - Integers are stored as `i64` (64-bit signed). Large unsigned integers beyond `i64::MAX` will lose precision.
   - Floats are stored as `f64` (IEEE 754). Values are limited to ~15 significant digits.

5. **Custom Serialization**: Serde's `#[serde(rename)]`, `#[serde(skip)]`, and other attributes are fully supported, allowing fine-grained control over serialization.

---

## 7. Comparison with JSON

| Feature | JSON | COSY |
|---------|------|------|
| Comments | ❌ | ✅ `// comment` |
| Unquoted keys | ❌ | ✅ `name: value` |
| Trailing commas | ❌ | ✅ `[1, 2,]` |
| Newline separators | ❌ | ✅ Can replace commas |
| Preserved key order | ❌ | ✅ Insertion-order maintained |
| Integer distinction | ❌ (all numbers) | ✅ `42` vs `3.14` |
| Null support | ✅ | ✅ |
| Strings | ✅ | ✅ |
| Arrays | ✅ | ✅ |
| Objects | ✅ | ✅ |
| Serde support | ⚠️ (custom impl) | ✅ Full integration |

---

## 8. Use Cases & Design Philosophy

## Best Use Cases

COSY excels as a **human-first configuration language**:

- **Application Config Files**: Server settings, database URLs, feature flags, logging levels
- **Build System Configs**: Makefiles, build pipelines, deployment manifests
- **Game/Asset Configs**: Level settings, character stats, world properties
- **Settings Files**: Any config where humans will write, review, and git-track the file
- **Documentation Examples**: Config files in READMEs and docs (human-readable by design)

### Why COSY for Configs?

| Need | Solution |
|------|----------|
| Comments in config | ✓ COSY (JSON ✗) |
| Readable by non-programmers | ✓ COSY (YAML is complex) |
| Easy to parse | ✓ COSY (YAML ambiguous) |
| Type clarity (int vs float) | ✓ COSY (JSON blurs this) |
| Key order preservation | ✓ COSY (JSON ✗) |
| Fast to implement | ✓ COSY (~500 LOC) |

## When NOT to Use COSY

These aren't limitations—they're intentional design choices:

**Don't use COSY for...**
1. **General data serialization** - If you need to serialize arbitrary Rust types (DateTime, UUID, custom structs with complex logic), use JSON or bincode. COSY targets *configuration*, not *serialization*.

2. **Preserving comments during roundtrip** - COSY strips comments during parsing (like all formats except YAML). This is intentional: comments are for *reading* configs, not preserving metadata.

3. **Non-string map keys** - Only `String` keys are supported. This keeps configs simple and auditable. If you need int/enum keys, you probably need a database, not a config file.

4. **Complex enum variants** - Only unit and newtype variants. Struct variants shouldn't appear in configs (put complex logic in code, not data files).

## The Philosophy

**COSY is opinionated about what configs should be:**
- Human-readable and hand-editable ✓
- Type-safe when deserialized ✓
- Auditable (no hidden complexity) ✓
- Simple to implement (no external deps for parsing) ✓

**COSY is NOT:**
- A replacement for general serialization (use JSON/bincode)
- A data interchange format (use JSON)
- A general-purpose markup language (use YAML for that complexity)

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
let result: Result<Config, _> = serde::from_str("{ port: \"not a number\" }");
// Error: Deserialization error: expected integer
```

---

## 10. Production Readiness

COSY is **production-ready for configuration files**:

✓ Comprehensive error messages with line/column info
✓ Full Serde integration for type-safe deserialization
✓ Key order preservation (critical for readable configs)
✓ Extensive test coverage (95+ tests)
✓ No external dependencies (fast, small, auditable)

If your use case is "I need a human-friendly config format", COSY is ready today.

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

- **1.7.0** - Added Comment Preservation: Comments are now preserved during parsing and round-trip serialization.
- **1.6.0** - Added Strict Mode Features: Typo suggestions ("Did you mean...?") and Deprecation warnings.
- **1.5.0** - Added Config Merging (`cosy::load_and_merge`) and Deep Merge logic.
- **1.4.0** - Added Config File Inclusion (`cosy::include`).
- **1.3.0** - Added Schema Validation (`cosy::schema::validate`).
- **1.2.0** - Added Environment Variable Interpolation (`${VAR}`) with type inference.
- **1.1.0** - Architectural overhaul: explicit newline tokenization for robust parsing
- **1.0.0** - Initial specification with full Serde support and comprehensive error handling

---

## 13. Future Roadmap

### Planned Features (High Priority)

These features would enhance COSY for production config management without changing the core format:

**1. Schema Validation** (Completed v1.3.0)
- ✅ Validate config files against a declarative schema
- ✅ Catch typos and type mismatches early
- ✅ Example: `cosy::validate(&config, &schema)?`

**2. Environment Variable Interpolation** (Completed v1.2.0)
- ✅ Reference environment variables in configs
- ✅ Example: `database_url: "${DB_URL}"` or `database_url: "$${DB_URL}"`
- ✅ Useful for secrets and environment-specific settings without duplicating configs

**3. Config File Inclusion** (Completed v1.4.0)
- ✅ Include other COSY files to avoid repetition
- ✅ Example: `include: "shared/logging.cosy"`
- ✅ Support for relative paths and overrides

**4. Strict Mode & Linting** (Completed v1.6.0)
- ✅ Flag unknown keys (catch typos: `debg: true` instead of `debug`)
- ✅ Warn about deprecated config keys
- ✅ Suggest corrections: "Unknown key 'port'; did you mean 'ports'?"
- ✅ Useful for catching accidental misconfigurations

**5. Comments Preservation** (Completed v1.7.0)
- ✅ Preserve comments during roundtrip serialization
- ✅ Comments attached to AST nodes
- ✅ Useful for programmatic config modification while maintaining documentation



### Considered (Lower Priority)

**CLI Tool** - A command-line utility for:
- Validating COSY files: `cosy validate config.cosy --schema schema.cosy`
- Pretty-printing: `cosy format config.cosy --indent 2`
- Converting to/from JSON: `cosy to-json config.cosy`
- Checking against schema: `cosy check config.cosy --schema config.schema`

**Custom Derive Macros** - `#[cosy(...)]` attributes for fine-grained control
- Would support: field validation, custom deserialization, computed fields
- Example: `#[cosy(validate = "port > 0 && port < 65536")]`

### Not Planned

**Complex Type Support** - COSY intentionally keeps types simple (primitives, strings, arrays, objects). Complex types belong in code or databases, not configs.

**Binary Data** - COSY is text-based for auditability and version control friendliness.

**Macro Language** - COSY is deliberately simple; complex logic belongs in application code.

---

### Contributing to the Roadmap

Have a feature idea? We'd love to hear it! Please open an issue describing:
- Your use case
- Why it would help (especially for configuration management)
- How it fits COSY's philosophy of simplicity and human-readability
