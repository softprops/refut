#![cfg(all(test, target_arch = "wasm32"))]

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use again::delay::Delay;

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn wait_two_seconds() {
    let _ = Delay::new(std::time::Duration::from_secs(2)).await;
}
