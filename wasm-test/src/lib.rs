use polars::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use serde_json::json;

#[wasm_bindgen_test]
fn launch_worker() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    let s: Series = [1, 2, 3].iter().collect();
    let s_squared: Series = s
    .i32()
    .expect("datatype mismatch")
        .into_iter()
        .map(|optional_v| {
            match optional_v {
                Some(v) => Some(v * v),
                None => None, // null value
            }
        })
        .collect();
    log::debug!("s: {:?}", s_squared);

    let b = s_squared.f64().unwrap();
}
