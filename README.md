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

Input from a large production-style export. In practice this kind of payload can easily be tens of thousands of lines long; the sample below is already long enough to show why reading the raw JSON directly is painful:

```text
{
  "export_id": "exp_2026_03_27_001",
  "generated_at": "2026-03-27T03:14:15Z",
  "tenant": {
    "id": "tenant_42",
    "name": "Acme Global",
    "plan": "enterprise",
    "regions": ["us-east-1", "eu-west-1", "ap-southeast-1"],
    "feature_flags": [
      "audit-trail",
      "advanced-billing",
      "fine-grained-rbac",
      "sso",
      "warehouse-sync"
    ]
  },
  "users": [
    {
      "id": "usr_001",
      "profile": {
        "email": "ada@acme.example",
        "display_name": "Ada Lovelace",
        "title": "Principal Engineer",
        "team": "Platform",
        "locale": "en-US",
        "timezone": "America/Los_Angeles",
        "phones": ["+1-415-555-0101", "+1-415-555-0102"]
      },
      "roles": ["admin", "billing", "support"],
      "projects": [
        {
          "project_id": "prj_alpha",
          "name": "Alpha",
          "status": "active",
          "environments": ["dev", "staging", "prod"],
          "budgets": {
            "monthly_limit": 20000,
            "currency": "USD"
          }
        },
        {
          "project_id": "prj_beta",
          "name": "Beta",
          "status": "paused",
          "environments": ["dev", "prod"],
          "budgets": {
            "monthly_limit": 5000,
            "currency": "USD"
          }
        }
      ],
      "devices": [
        {
          "device_id": "dev_mac_01",
          "type": "laptop",
          "os": "macOS",
          "last_seen_at": "2026-03-27T02:58:10Z",
          "ip_addresses": ["203.0.113.10", "203.0.113.11"]
        },
        {
          "device_id": "dev_phone_01",
          "type": "phone",
          "os": "iOS",
          "last_seen_at": "2026-03-26T23:41:05Z",
          "ip_addresses": ["198.51.100.7"]
        }
      ],
      "activity": [
        {
          "event_id": "evt_9001",
          "kind": "login",
          "status": "success",
          "source": "web",
          "context": {
            "ip": "203.0.113.10",
            "country": "US",
            "mfa": true
          }
        },
        {
          "event_id": "evt_9002",
          "kind": "export_csv",
          "status": "success",
          "source": "api",
          "context": {
            "ip": "203.0.113.11",
            "country": "US",
            "mfa": true
          }
        }
      ],
      "billing": {
        "cost_center": "ENG-PLATFORM",
        "invoice_email": "ops@acme.example",
        "active_subscriptions": ["analytics", "support-plus", "warehouse-sync"]
      },
      "security": {
        "mfa_enabled": true,
        "password_last_rotated_at": "2026-02-18T11:20:00Z",
        "recovery_codes_remaining": 8
      }
    },
    {
      "id": "usr_002",
      "profile": {
        "email": "grace@acme.example",
        "display_name": "Grace Hopper",
        "title": "Staff Data Engineer",
        "team": "Data",
        "locale": "en-US",
        "timezone": "America/New_York"
      },
      "roles": ["editor", "viewer"],
      "projects": [
        {
          "project_id": "prj_gamma",
          "name": "Gamma",
          "status": "active",
          "environments": ["dev", "prod"],
          "budgets": {
            "monthly_limit": 12000,
            "currency": "USD"
          }
        }
      ],
      "devices": [
        {
          "device_id": "dev_linux_01",
          "type": "desktop",
          "os": "Linux",
          "last_seen_at": "2026-03-27T01:03:44Z",
          "ip_addresses": ["192.0.2.44"]
        }
      ],
      "activity": [
        {
          "event_id": "evt_9100",
          "kind": "query_saved",
          "status": "success",
          "source": "web",
          "context": {
            "ip": "192.0.2.44",
            "country": "US",
            "mfa": true
          }
        }
      ],
      "security": {
        "mfa_enabled": true,
        "password_last_rotated_at": "2026-01-12T09:05:12Z",
        "recovery_codes_remaining": 3
      }
    }
  ],
  "jobs": [
    {
      "job_id": "job_1001",
      "kind": "warehouse_sync",
      "state": "running",
      "priority": 7,
      "queue": "critical",
      "attempts": [
        {
          "attempt": 1,
          "started_at": "2026-03-27T03:10:00Z",
          "worker": "sync-worker-01"
        }
      ]
    },
    {
      "job_id": "job_1002",
      "kind": "invoice_generation",
      "state": "failed",
      "priority": 5,
      "queue": "billing",
      "attempts": [
        {
          "attempt": 1,
          "started_at": "2026-03-27T02:00:00Z",
          "finished_at": "2026-03-27T02:01:14Z",
          "worker": "billing-worker-03",
          "error": {
            "code": "PDF_TIMEOUT",
            "message": "Timed out waiting for renderer"
          }
        },
        {
          "attempt": 2,
          "started_at": "2026-03-27T02:03:10Z",
          "finished_at": "2026-03-27T02:04:11Z",
          "worker": "billing-worker-04",
          "error": {
            "code": "PDF_TIMEOUT",
            "message": "Timed out waiting for renderer"
          }
        }
      ]
    }
  ],
  "audit_log": [
    {
      "id": "log_0001",
      "actor": "usr_001",
      "action": "project.update",
      "resource": "prj_alpha",
      "changes": {
        "before": {"status": "paused"},
        "after": {"status": "active"}
      }
    },
    {
      "id": "log_0002",
      "actor": "system",
      "action": "job.retry",
      "resource": "job_1002",
      "changes": {
        "before": {"attempt": 1},
        "after": {"attempt": 2}
      }
    }
  ],
  "analytics": {
    "dashboards": 18,
    "saved_queries": 249,
    "warehouse_bytes_scanned_30d": 9827349821,
    "top_tables": ["events_raw", "billing_invoices", "user_sessions", "audit_log"]
  }
}
```

Output after running `jshape`:

```text
{
  "export_id": "exp_2026_03_27_001",
  "generated_at": "2026-03-27T03:14:15Z",
  "tenant": {
    "id": "tenant_42",
    "name": "Acme Global",
    "plan": "enterprise",
    "regions": [
      "us-east-1", "eu-west-1", "ap-southeast-1"
    ],
    "feature_flags": [
      "audit-trail", "advanced-billing", "fine-grained-rbac", "sso", "warehouse-sync"
    ]
  },
  "users": [
    {
      "id": "usr_001",
      "profile": {
        "email": "ada@acme.example",
        "display_name": "Ada Lovelace",
        "title": "Principal Engineer",
        "team": "Platform",
        "locale": "en-US",
        "timezone": "America/Los_Angeles",
        "phones"?: [
          "+1-415-555-0101", "+1-415-555-0102"
        ]
      },
      "roles": [
        "admin", "billing", "support", "editor", "viewer"
      ],
      "projects": [
        {
          "project_id": "prj_alpha",
          "name": "Alpha",
          "status": "active", "paused",
          "environments": [
            "dev", "staging", "prod"
          ],
          "budgets": {
            "monthly_limit": 20000, 5000, 12000,
            "currency": "USD"
          }
        },
      ...  // 2 items
      ],
      "devices": [
        {
          "device_id": "dev_mac_01",
          "type": "laptop",
          "os": "macOS",
          "last_seen_at": "2026-03-27T02:58:10Z",
          "ip_addresses": [
            "203.0.113.10", "203.0.113.11", "198.51.100.7", "192.0.2.44"
          ]
        },
      ...  // 2 items
      ],
      "activity": [
        {
          "event_id": "evt_9001",
          "kind": "login",
          "status": "success",
          "source": "web",
          "context": {
            "ip": "203.0.113.10",
            "country": "US",
            "mfa": true
          }
        },
      ...  // 2 items
      ],
      "billing"?: {
        "cost_center": "ENG-PLATFORM",
        "invoice_email": "ops@acme.example",
        "active_subscriptions": [
          "analytics", "support-plus", "warehouse-sync"
        ]
      },
      "security": {
        "mfa_enabled": true,
        "password_last_rotated_at": "2026-02-18T11:20:00Z",
        "recovery_codes_remaining": 8, 3
      }
    },
  ...  // 2 items
  ],
  "jobs": [
    {
      "job_id": "job_1001",
      "kind": "warehouse_sync",
      "state": "running",
      "priority": 7, 5,
      "queue": "critical", "billing",
      "attempts": [
        {
          "attempt": 1, 2,
          "started_at": "2026-03-27T03:10:00Z",
          "finished_at"?: "2026-03-27T02:01:14Z",
          "worker": "sync-worker-01", "billing-worker-03", "billing-worker-04",
          "error"?: {
            "code": "PDF_TIMEOUT",
            "message": "Timed out waiting for renderer"
          }
        },
      ...  // 2 items
      ]
    },
  ...  // 2 items
  ],
  "audit_log": [
    {
      "id": "log_0001",
      "actor": "usr_001", "system",
      "action": "project.update", "job.retry",
      "resource": "prj_alpha", "job_1002",
      "changes": {
        "before": {
          "status"?: "paused",
          "attempt"?: 1
        },
        "after": {
          "status"?: "active",
          "attempt"?: 2
        }
      }
    },
  ...  // 2 items
  ],
  "analytics": {
    "dashboards": 18,
    "saved_queries": 249,
    "warehouse_bytes_scanned_30d": 9827349821,
    "top_tables": [
      "events_raw", "billing_invoices", "user_sessions", "audit_log"
    ]
  }
}
```

That is the core use case: take a huge payload that is exhausting to read directly, then collapse it into a compact structural outline that still preserves enough concrete examples to understand the data.

## Notes

- Output is JSON-like and stable for inspection, but it is not guaranteed to be valid JSON in every mode.
- Optional fields are rendered with a trailing `?`.
- Large arrays are summarized instead of printing every element in full.
- This crate currently relies on `json-repair = 0.4.0`, which requires a nightly-compatible build environment or `RUSTC_BOOTSTRAP=1`.
