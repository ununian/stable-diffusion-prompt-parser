pub mod parser;
mod utils;

use parser::{cst::PromptCST, parser::ParseResult};
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate pest_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Range = [number, number];
export interface BracketKind {
    Bracket: [type: 'S' | 'M' | 'L',level: number]
}

export type Kinds = "TagStatement" | BracketKind | "SingleTag" | "Word";	

export interface PromptCST {
    kind: Kinds;
    text: string;
    range: [start: number,end: number];
    inner: PromptCST[];
    bracket_modifier: string;
}
"#;

#[wasm_bindgen]
pub fn cst(input: &str) -> Result<JsValue, JsValue> {
    let result = ParseResult::parse(input);
    let pairs = result.unwrap().pairs;
    let cst = PromptCST::parse(pairs.clone());
    Ok(serde_wasm_bindgen::to_value(&cst)?)
}
