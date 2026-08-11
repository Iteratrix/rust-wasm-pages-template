//! WASM bridge: JSON-in/JSON-out API over the core crate.
//!
//! Thin by design: (de)serialization lives here, logic lives in core. A
//! string API keeps the JS side trivial (`JSON.parse` and go). If the
//! boundary grows rich enough that stringly typing hurts, upgrade to
//! `tsify` for generated TypeScript types.

use serde::Serialize;
use wasm_bindgen::prelude::*;

use {{crate_name}}_core::Summary;

#[derive(Serialize)]
struct SummaryJson {
    len: usize,
    utf8: bool,
    lines: Option<usize>,
}

impl From<Summary> for SummaryJson {
    fn from(summary: Summary) -> Self {
        let Summary { len, utf8, lines } = summary;
        Self { len, utf8, lines }
    }
}

/// Summarize a file's bytes; returns a JSON `SummaryJson`.
///
/// # Panics
///
/// Panics only if JSON serialization fails, which cannot happen for this
/// type.
#[wasm_bindgen]
#[must_use]
pub fn summarize(bytes: &[u8]) -> String {
    let summary = SummaryJson::from({{crate_name}}_core::summarize(bytes));
    serde_json::to_string(&summary).expect("Summary serializes")
}
