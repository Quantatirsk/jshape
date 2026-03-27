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

As a Rust library:

```toml
[dependencies]
jshape = "0.1.0"
```

As a Rust CLI:

```bash
cargo install jshape
```

As a Python package:

```bash
pip install jshape
```

As a Node.js package:

```bash
npm install @quanthub/jshape
```

## Publish

This repository ships with separate GitHub Actions workflows for Rust, Python, and Node.js publishing.

- Rust / crates.io:
  Add a repository secret named `CARGO_REGISTRY_TOKEN`, then run `Publish Rust Crate`.
- Python / PyPI:
  Configure PyPI trusted publishing for repository `Quantatirsk/jshape` and workflow file `.github/workflows/publish-python.yml`, then run `Publish Python Package`.
- Node.js / npm:
  Configure npm trusted publishing for package `@quanthub/jshape`, repository `Quantatirsk/jshape`, and workflow file `.github/workflows/publish-node.yml`, then run `Publish Node Package`.

All three workflows also publish automatically when you push a version tag such as `v0.1.0`.

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

## Rust Usage

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

## Python Usage

```python
import jshape

outline = jshape.analyze_json(
    '{"user":{"name":"Ada"},"events":[{"id":1},{"id":2,"amount":19.9}]}',
    True,
)

print(outline)
```

## Node.js Usage

```js
const { analyzeJson } = require("@quanthub/jshape");

const outline = analyzeJson(
  '{"user":{"name":"Ada"},"events":[{"id":1},{"id":2,"amount":19.9}]}',
  true,
);

console.log(outline);
```

The Node.js package is built from the same Rust core, but distributed as a WebAssembly package so it installs cleanly through npm.

## Example

The painful case is usually not a tiny nested object. It is a giant export where one array contains thousands of items with almost the same structure, making the raw JSON long and hard to scan.

Below is a trimmed excerpt from a much larger analytics export. In the real file, the `events` array contains tens of thousands of similarly shaped records:

```text
{
  "export_id": "exp_2026_03_27_001",
  "generated_at": "2026-03-27T03:14:15Z",
  "tenant_id": "tenant_42",
  "events": [
    {
      "event_id": "evt_000001",
      "session_id": "sess_a1",
      "user_id": "usr_1001",
      "event_type": "page_view",
      "source": "web",
      "timestamp": "2026-03-27T03:00:01Z",
      "request": {
        "method": "GET",
        "path": "/dashboard",
        "status": 200,
        "duration_ms": 42
      },
      "device": {
        "os": "macOS",
        "browser": "Chrome",
        "locale": "en-US"
      },
      "geo": {
        "country": "US",
        "region": "CA",
        "city": "San Francisco"
      },
      "metrics": {
        "cpu_ms": 12,
        "db_rows": 18,
        "cache_hit": true
      },
      "tags": ["prod", "dashboard", "page_view"]
    },
    {
      "event_id": "evt_000002",
      "session_id": "sess_a1",
      "user_id": "usr_1001",
      "event_type": "page_view",
      "source": "web",
      "timestamp": "2026-03-27T03:00:03Z",
      "request": {
        "method": "GET",
        "path": "/dashboard/usage",
        "status": 200,
        "duration_ms": 57
      },
      "device": {
        "os": "macOS",
        "browser": "Chrome",
        "locale": "en-US"
      },
      "geo": {
        "country": "US",
        "region": "CA",
        "city": "San Francisco"
      },
      "metrics": {
        "cpu_ms": 19,
        "db_rows": 44,
        "cache_hit": true
      },
      "tags": ["prod", "dashboard", "page_view"]
    },
    {
      "event_id": "evt_000003",
      "session_id": "sess_b9",
      "user_id": "usr_2048",
      "event_type": "api_call",
      "source": "api",
      "timestamp": "2026-03-27T03:00:04Z",
      "request": {
        "method": "POST",
        "path": "/v1/reports/query",
        "status": 200,
        "duration_ms": 183
      },
      "device": {
        "os": "Linux",
        "browser": "curl",
        "locale": "en-US"
      },
      "geo": {
        "country": "DE",
        "region": "BE",
        "city": "Berlin"
      },
      "metrics": {
        "cpu_ms": 98,
        "db_rows": 1200,
        "cache_hit": false
      },
      "tags": ["prod", "reports", "api"],
      "error": {
        "code": "RATE_LIMIT_NEAR",
        "retryable": true
      }
    },
    {
      "event_id": "evt_000004",
      "session_id": "sess_c2",
      "user_id": "usr_3099",
      "event_type": "page_view",
      "source": "web",
      "timestamp": "2026-03-27T03:00:05Z",
      "request": {
        "method": "GET",
        "path": "/billing/invoices",
        "status": 200,
        "duration_ms": 61
      },
      "device": {
        "os": "Windows",
        "browser": "Edge",
        "locale": "en-GB"
      },
      "geo": {
        "country": "GB",
        "region": "LND",
        "city": "London"
      },
      "metrics": {
        "cpu_ms": 21,
        "db_rows": 72,
        "cache_hit": true
      },
      "tags": ["prod", "billing", "page_view"]
    },
    ... thousands more records with the same overall shape ...
  ],
  "aggregates": {
    "event_count": 48762,
    "unique_users": 913,
    "time_range": {
      "from": "2026-03-27T00:00:00Z",
      "to": "2026-03-27T03:14:15Z"
    }
  }
}
```

Output after running `jshape`:

```text
{
  "export_id": "exp_2026_03_27_001",
  "generated_at": "2026-03-27T03:14:15Z",
  "tenant_id": "tenant_42",
  "events": [
    {
      "event_id": "evt_000001",
      "session_id": "sess_a1",
      "user_id": "usr_1001",
      "event_type": "page_view", "api_call",
      "source": "web", "api",
      "timestamp": "2026-03-27T03:00:01Z",
      "request": {
        "method": "GET", "POST",
        "path": "/dashboard", "/dashboard/usage", "/v1/reports/query", "/billing/invoices",
        "status": 200,
        "duration_ms": 42, 57, 183, 61
      },
      "device": {
        "os": "macOS", "Linux", "Windows",
        "browser": "Chrome", "curl", "Edge",
        "locale": "en-US", "en-GB"
      },
      "geo": {
        "country": "US", "DE", "GB",
        "region": "CA", "BE", "LND",
        "city": "San Francisco", "Berlin", "London"
      },
      "metrics": {
        "cpu_ms": 12, 19, 98, 21,
        "db_rows": 18, 44, 1200, 72,
        "cache_hit": bool
      },
      "tags": [
        "prod", "dashboard", "page_view", "reports", "api", "billing"
      ],
      "error"?: {
        "code": "RATE_LIMIT_NEAR",
        "retryable": true
      }
    },
  ...  // 48762 items
  ],
  "aggregates": {
    "event_count": 48762,
    "unique_users": 913,
    "time_range": {
      "from": "2026-03-27T00:00:00Z",
      "to": "2026-03-27T03:14:15Z"
    }
  }
}
```

The difference is the point of the tool: the raw input repeats the same object shape thousands of times, while the output keeps one representative structure, marks optional fields, preserves a few concrete values, and tells you how large the array really is.

## Notes

- Output is JSON-like and stable for inspection, but it is not guaranteed to be valid JSON in every mode.
- Optional fields are rendered with a trailing `?`.
- Large arrays are summarized instead of printing every element in full.
- This crate currently relies on `json-repair = 0.4.0`, which requires a nightly-compatible build environment or `RUSTC_BOOTSTRAP=1`.
