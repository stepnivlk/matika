use matika_interpreter::Matika as Libmatika;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Matika {
    interpreter: Libmatika,
}

#[wasm_bindgen]
impl Matika {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { interpreter: Libmatika::new() }
    }

    #[wasm_bindgen]
    pub fn eval(&mut self, txt: String) -> f64 {
        self.interpreter.eval(txt).into()
    }
}