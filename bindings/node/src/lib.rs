use ::jshape as core;
use wasm_bindgen::prelude::*;

fn analyze_json_impl(input: &str, show_examples: bool) -> Result<String, String> {
    core::analyze_json(input, show_examples)
}

#[wasm_bindgen(js_name = analyzeJson)]
pub fn analyze_json(input: &str, show_examples: bool) -> Result<String, JsValue> {
    analyze_json_impl(input, show_examples).map_err(|err| JsValue::from_str(&err))
}

#[cfg(test)]
mod tests {
    use super::analyze_json_impl;

    #[test]
    fn analyzes_json_from_node_binding() {
        let output = analyze_json_impl(r#"{"user":{"name":"Ada"}}"#, true).unwrap();
        assert!(output.contains("\"user\""));
        assert!(output.contains("\"name\""));
    }
}
