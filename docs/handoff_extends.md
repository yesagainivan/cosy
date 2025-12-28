# Handoff: `extends` Keyword for Cosy

**Status**: Proposal / Hand-off
**Goal**: Implement a semantic `extends` keyword to clarify intent when inheriting from base configurations/themes.

## Context
Currently, `cosy` supports `include: "path/to/file"` which merges the included file's content into the current object. This effectively enables inheritance (since the includer can override the included values).
 However, `include` implies "fragment composition", whereas `extends` implies "inheritance/specialization".
 Users (especially for themes) prefer `extends` for base themes.

## Proposal
Add `extends` as a top-level keyword (or valid field) that behaves similarly to `include`, but with specific semantics:
1.  **Top-Level Only**: `extends` typically makes sense at the root of a file (though nested could work).
2.  **Order of Operations**:
    *   Load `extends` target first (Base).
    *   Load current file (Child).
    *   Merge Child *over* Base.
    *   (This is the same as `include` if `include` is processed before other fields).

## Implementation Steps (in `cosy` lib)

1.  **Parser Update**:
    *   Ensure `extends` is a reserved or recognized key if necessary (though standard string key is fine).

2.  **Resolution Logic (`src/include.rs` or similar)**:
    *   In `resolve` function:
        *   Check for `extends` key.
        *   If present, recursively resolve it *before* `include`?
        *   Usually `extends` is the "base", and `include` might be mixins.
        *   Suggested Order: `extends` -> `include` -> `local fields`.

3.  **Deprecation/Warning (Optional)**:
    *   If using `extends`, ensure it doesn't conflict with `include` order if both are present.

## Example Usage

```cosy
// my-theme.cosy
extends: "defaults.cosy"

colors: {
    primary: "red" // Overrides defaults.cosy
}
```

## Workaround (Current Status)
Users can achieve this today using `include`:
```cosy
include: "defaults.cosy"
colors: { primary: "red" }
```
This is fully functional in `themy` v0.1.0 using `cosy` v1.7.0.
