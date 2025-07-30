use wasm_bindgen::prelude::*;

// Import our modules
pub mod abilities;
pub mod combat;
pub mod commitment;
pub mod game_state;
pub mod league;

// Re-export public types
pub use combat::{
    generate_army_from_cashu_c_value, generate_units_from_token_secret, process_combat,
};
pub use commitment::*;
pub use game_state::{Ability, RoundResult, Unit};

// WASM initialization
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Use wee_alloc as the global allocator for smaller WASM binary
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
}

// WASM exports for JavaScript/TypeScript
#[wasm_bindgen]
pub fn wasm_generate_units_from_token_secret(token_secret: &str, league_id: u8) -> JsValue {
    let units = combat::generate_units_from_token_secret(token_secret, league_id);
    serde_wasm_bindgen::to_value(&units).unwrap()
}

#[wasm_bindgen]
pub fn wasm_process_combat(
    unit1_js: JsValue,
    unit2_js: JsValue,
    player1_npub: &str,
    player2_npub: &str,
) -> JsValue {
    let unit1: Unit = serde_wasm_bindgen::from_value(unit1_js).unwrap();
    let unit2: Unit = serde_wasm_bindgen::from_value(unit2_js).unwrap();

    let result = combat::process_combat(unit1, unit2, player1_npub, player2_npub).unwrap();
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn wasm_apply_league_modifiers(base_unit_js: JsValue, league_id: u8) -> JsValue {
    let mut unit: Unit = serde_wasm_bindgen::from_value(base_unit_js).unwrap();
    league::apply_modifiers(&mut unit, league_id);
    serde_wasm_bindgen::to_value(&unit).unwrap()
}

// Test function for WASM module verification
#[wasm_bindgen]
pub fn wasm_test_connection() -> String {
    "WASM shared game logic loaded successfully".to_string()
}

// Console logging helper for WASM debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[allow(unused_imports)]
pub(crate) use console_log;
