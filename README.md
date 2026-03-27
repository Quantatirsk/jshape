# jshape

`jshape` repairs malformed JSON input and renders a stable, human-readable structural outline.

It is designed for large or messy JSON payloads where you want to understand the shape of the data quickly instead of reading the full document.

## What It Does

- Repairs malformed JSON before parsing
- Extracts nested object and array structure
- Preserves example values for scalar fields
- Marks optional object fields with `?`
- Preserves object key order from the input
- Ships with a short CLI command: `jshape`

## Install

As a library:

```toml
[dependencies]
jshape = "0.1.0"
```

As a CLI:

```bash
cargo install jshape
```

## Publish

This repository ships with a GitHub Actions workflow for crates.io publishing.

1. Add a repository secret named `CARGO_REGISTRY_TOKEN`
2. Open the `Publish Crate` workflow in GitHub Actions
3. Click `Run workflow`

The same workflow also publishes automatically when you push a version tag such as `v0.1.0`.

## CLI Usage

Read from a file:

```bash
jshape payload.json
```

Read from stdin:

```bash
cat payload.json | jshape
```

Show types instead of example values:

```bash
jshape --no-examples payload.json
```

## Library Usage

```rust
use jshape::analyze_json;

fn main() {
    let input = r#"{
        'user': {
            "name": "Ada",
            "roles": ["admin", "editor"]
        }
    }"#;

    let outline = analyze_json(input, true).unwrap();
    println!("{}", outline);
}
```

If you need lower-level access, the crate also exposes:

- `repair_and_parse_json`
- `extract_schema`
- `format_schema`
- `Schema`

## Example

Input:

```text
{
  'user': {
    "name": "Ada",
    "roles": ["admin", "editor"]
  },
  "events": [
    {"id": 1, "type": "login"},
    {"id": 2, "type": "purchase", "amount": 19.9}
  ]
}
```

Output:

```text
{
  "user": {
    "name": "Ada",
    "roles": [
      "admin", "editor"
    ]
  },
  "events": [
    {
      "id": 1,
      "type": "login",
      "amount"?: 19.9
    },
  ...  // 2 items
  ]
}
```

## Notes

- Output is JSON-like and stable for inspection, but it is not guaranteed to be valid JSON in every mode.
- Optional fields are rendered with a trailing `?`.
- Large arrays are summarized instead of printing every element in full.
- This crate currently relies on `json-repair = 0.4.0`, which requires a nightly-compatible build environment or `RUSTC_BOOTSTRAP=1`.
