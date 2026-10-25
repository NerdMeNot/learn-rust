use wasm_bindgen::prelude::*;

// A JavaScript function Rust can call: console.log.
#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

// &str in, String out: the generated glue copies the text across for you.
#[wasm_bindgen]
pub fn shout(text: &str) -> String {
    log(&format!("shout got {} bytes", text.len()));
    format!("{}!", text.to_uppercase())
}

// A slice in; JavaScript passes an Int32Array.
#[wasm_bindgen]
pub fn sum(values: &[i32]) -> i64 {
    values.iter().map(|&v| v as i64).sum()
}

// Err becomes a thrown JavaScript Error.
#[wasm_bindgen]
pub fn parse(text: &str) -> Result<i32, JsError> {
    text.trim().parse::<i32>().map_err(|e| JsError::new(&e.to_string()))
}

// A struct becomes a JavaScript class that owns Rust memory.
#[wasm_bindgen]
pub struct Counter {
    count: u32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter { count: 0 }
    }

    pub fn bump(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
