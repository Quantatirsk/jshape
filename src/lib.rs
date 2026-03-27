use indexmap::IndexMap;
use json_repair::repair_json_string;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Schema {
    Null,
    Bool(Option<bool>),
    Int(Vec<i64>),
    Float(Vec<f64>),
    Str(Vec<String>),
    Array(Box<Schema>, usize, Option<HashSet<String>>),
    Object(IndexMap<String, Schema>, HashSet<String>),
}

pub fn repair_and_parse_json(input: &str) -> Result<Value, String> {
    repair_json_string(input).map_err(|err| format!("Failed to parse or repair JSON: {}", err))
}

pub fn extract_schema(value: &Value) -> Schema {
    match value {
        Value::Null => Schema::Null,
        Value::Bool(b) => Schema::Bool(Some(*b)),
        Value::Number(n) if n.is_i64() => Schema::Int(vec![n.as_i64().unwrap()]),
        Value::Number(n) => Schema::Float(vec![n.as_f64().unwrap()]),
        Value::String(s) => Schema::Str(vec![s.clone()]),
        Value::Array(arr) => {
            if arr.is_empty() {
                return Schema::Array(Box::new(Schema::Null), 0, None);
            }
            let mut item_schema = extract_schema(&arr[0]);

            match &mut item_schema {
                Schema::Str(examples) => {
                    for v in arr.iter().skip(1) {
                        if let Value::String(s) = v {
                            if !examples.contains(s) {
                                examples.push(s.clone());
                            }
                        }
                    }
                }
                Schema::Int(examples) => {
                    for v in arr.iter().skip(1) {
                        if let Value::Number(n) = v {
                            if let Some(i) = n.as_i64() {
                                if !examples.contains(&i) {
                                    examples.push(i);
                                }
                            }
                        }
                    }
                }
                Schema::Float(examples) => {
                    for v in arr.iter().skip(1) {
                        if let Value::Number(n) = v {
                            if let Some(f) = n.as_f64() {
                                if !examples.contains(&f) {
                                    examples.push(f);
                                }
                            }
                        }
                    }
                }
                Schema::Bool(_) => {
                    item_schema = Schema::Bool(None);
                }
                _ => {
                    for v in arr.iter().skip(1) {
                        let other = extract_schema(v);
                        merge_schema(&mut item_schema, &other);
                    }
                }
            }

            let opt_keys = match &item_schema {
                Schema::Object(_, opt) if !opt.is_empty() => Some(opt.clone()),
                _ => None,
            };

            Schema::Array(Box::new(item_schema), arr.len(), opt_keys)
        }
        Value::Object(map) => {
            let mut fields = IndexMap::new();
            for (k, v) in map {
                fields.insert(k.clone(), extract_schema(v));
            }
            Schema::Object(fields, HashSet::new())
        }
    }
}

pub fn format_schema(schema: &Schema, show_examples: bool) -> String {
    render_root(schema, show_examples)
}

pub fn analyze_json(input: &str, show_examples: bool) -> Result<String, String> {
    let value = repair_and_parse_json(input)?;
    let schema = extract_schema(&value);
    Ok(format_schema(&schema, show_examples))
}

fn merge_schema(a: &mut Schema, b: &Schema) {
    use Schema::*;
    match (&mut *a, b) {
        (Int(_), Float(_)) => {
            *a = Float(vec![]);
        }
        (Array(item_a, len_a, opt_a), Array(item_b, len_b, opt_b)) => {
            *len_a = (*len_a).max(*len_b);
            if let (Some(opt_a), Some(opt_b)) = (opt_a, opt_b) {
                let intersection: HashSet<_> = opt_a.intersection(opt_b).cloned().collect();
                *opt_a = intersection;
            }
            merge_schema(item_a, item_b);
        }
        (Object(fields_a, opt_a), Object(fields_b, opt_b)) => {
            for (k, v) in fields_b {
                if let Some(va) = fields_a.get_mut(k) {
                    merge_schema(va, v);
                } else {
                    opt_a.insert(k.clone());
                    fields_a.insert(k.clone(), v.clone());
                }
            }
            let keys_in_b: HashSet<_> = fields_b.keys().cloned().collect();
            for k in fields_a.keys().cloned().collect::<Vec<_>>() {
                if !keys_in_b.contains(&k) {
                    opt_a.insert(k);
                }
            }
            for k in opt_b {
                opt_a.insert(k.clone());
            }
        }
        _ => {}
    }
}

fn render_root(schema: &Schema, show_examples: bool) -> String {
    match schema {
        Schema::Object(fields, opt_fields) => {
            let mut lines = vec!["{".to_string()];
            let entry_count = fields.len();
            for (index, (k, v)) in fields.iter().enumerate() {
                let key_display = format_key(k, opt_fields.contains(k));
                let val_str = format_field_value(v, 1, 2, show_examples);
                let suffix = if index + 1 < entry_count { "," } else { "" };
                lines.push(format!("  {}: {}{}", key_display, val_str, suffix));
            }
            lines.push("}".to_string());
            lines.join("\n")
        }
        _ => format_schema_at_depth(schema, 1, 2, show_examples),
    }
}

fn format_scalar_value(schema: &Schema, show_examples: bool) -> Option<String> {
    match schema {
        Schema::Null => Some("null".to_string()),
        Schema::Bool(b) => Some(if show_examples {
            if let Some(val) = b {
                val.to_string()
            } else {
                "bool".to_string()
            }
        } else {
            "bool".to_string()
        }),
        Schema::Int(examples) => Some(if show_examples && !examples.is_empty() {
            examples.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
        } else {
            "int".to_string()
        }),
        Schema::Float(examples) => Some(if show_examples && !examples.is_empty() {
            examples.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")
        } else {
            "float".to_string()
        }),
        Schema::Str(examples) => Some(if show_examples && !examples.is_empty() {
            examples.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(", ")
        } else {
            "str".to_string()
        }),
        _ => None,
    }
}

fn format_key(key: &str, is_optional: bool) -> String {
    let quoted = serde_json::to_string(key).expect("schema keys should serialize to JSON strings");
    if is_optional {
        format!("{}?", quoted)
    } else {
        quoted
    }
}

fn format_block_field_value(schema: &Schema, depth: usize, indent: usize, show_examples: bool) -> String {
    let formatted = format_schema_at_depth(schema, depth, indent, show_examples);
    let pad = " ".repeat(depth * indent);

    let mut lines = formatted.lines();
    let first_line = lines
        .next()
        .map(|line| line.strip_prefix(&pad).unwrap_or(line).to_string())
        .unwrap_or_default();

    let mut result = first_line;
    for line in lines {
        result.push('\n');
        result.push_str(line);
    }
    result
}

fn format_array(
    item: &Schema,
    len: usize,
    opt_keys: Option<&HashSet<String>>,
    depth: usize,
    indent: usize,
    show_examples: bool,
) -> String {
    let pad = " ".repeat(depth * indent);
    let inner_pad = " ".repeat((depth + 1) * indent);

    match item {
        Schema::Object(fields, obj_opt) => {
            let mut lines = vec![format!("{}[", pad)];
            if len > 0 {
                lines.push(format!("{}{{", inner_pad));
                let entry_count = fields.len();
                for (index, (k, v)) in fields.iter().enumerate() {
                    let is_optional = obj_opt.contains(k)
                        || opt_keys.is_some_and(|keys| keys.contains(k));
                    let val_str = format_field_value(v, depth + 2, indent, show_examples);
                    let suffix = if index + 1 < entry_count { "," } else { "" };
                    lines.push(format!(
                        "{}{}: {}{}",
                        " ".repeat((depth + 2) * indent),
                        format_key(k, is_optional),
                        val_str,
                        suffix
                    ));
                }
                if len > 1 {
                    if show_examples {
                        lines.push(format!("{}}},", inner_pad));
                        lines.push(format!(
                            "{}...  // {} {}",
                            pad,
                            len,
                            if len == 1 { "item" } else { "items" }
                        ));
                    } else {
                        lines.push(format!("{}}}", inner_pad));
                    }
                } else {
                    lines.push(format!("{}}}", inner_pad));
                }
            }
            lines.push(format!("{}]", pad));
            lines.join("\n")
        }
        Schema::Array(_, _, _) => {
            let mut lines = vec![format!("{}[", pad)];
            let inner = format_schema_at_depth(item, depth + 1, indent, show_examples);
            for line in inner.lines() {
                lines.push(line.to_string());
            }
            lines.push(format!("{}]", pad));
            lines.join("\n")
        }
        _ => {
            let inner = format_scalar_value(item, show_examples)
                .expect("array primitive branch should only receive scalar item schemas");
            format!("{}[\n{}  {}\n{}]", pad, pad, inner, pad)
        }
    }
}

fn format_schema_at_depth(schema: &Schema, depth: usize, indent: usize, show_examples: bool) -> String {
    let pad = " ".repeat(depth * indent);
    if let Some(value) = format_scalar_value(schema, show_examples) {
        return format!("{}{}", pad, value);
    }

    match schema {
        Schema::Array(item, len, opt_keys) => {
            format_array(item, *len, opt_keys.as_ref(), depth, indent, show_examples)
        }
        Schema::Object(fields, opt_fields) => {
            let mut lines = vec![format!("{}{{", pad)];
            let entry_count = fields.len();
            for (index, (k, v)) in fields.iter().enumerate() {
                let val_str = format_field_value(v, depth + 1, indent, show_examples);
                let suffix = if index + 1 < entry_count { "," } else { "" };
                lines.push(format!(
                    "{}{}: {}{}",
                    " ".repeat((depth + 1) * indent),
                    format_key(k, opt_fields.contains(k)),
                    val_str,
                    suffix
                ));
            }
            lines.push(format!("{}}}", pad));
            lines.join("\n")
        }
        _ => unreachable!("non-scalar schemas are handled above"),
    }
}

fn format_field_value(schema: &Schema, depth: usize, indent: usize, show_examples: bool) -> String {
    if let Some(value) = format_scalar_value(schema, show_examples) {
        return value;
    }

    match schema {
        Schema::Object(_, _) => format_block_field_value(schema, depth, indent, show_examples),
        Schema::Array(_, _, _) => format_block_field_value(schema, depth, indent, show_examples),
        _ => unreachable!("scalar schemas are returned above"),
    }
}
