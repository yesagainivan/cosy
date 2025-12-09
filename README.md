# COSY Format Specification

**COSY** = **C**omfortable **O**bject **S**yntax, **Y**ay!

A human-friendly configuration format designed to be more pleasant to write and read than JSON, while maintaining simplicity and parsability.

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

## 6. Comparison with JSON

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

---

## 7. Error Reporting

The COSY parser provides detailed error messages with **line and column information**:

```
Parse error at line 3, column 15: Expected ':' after object key
```

This helps users quickly locate and fix issues in their configuration files.

---

## 8. Use Cases

COSY is ideal for:
- **Configuration files** (better than JSON, simpler than YAML)
- **Data serialization** (more human-friendly than JSON)
- **Game assets** (fast to write by hand)
- **Build system configs** (comments for documentation)

---

## 9. ABNF Grammar (Informal)

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

## 10. Version History

- **1.0.0** (Current) - Initial specification
