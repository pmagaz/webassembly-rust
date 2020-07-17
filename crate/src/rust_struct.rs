#[wasm_bindgen]
pub struct RustStruct {
  id: i32,
}

#[wasm_bindgen]
impl RustStruct {
  //Esta macro nos permite usar el metodo new como constructor en JavaScript para hacer un new RustStruct
  #[wasm_bindgen(constructor)]
  pub fn new(val: i32) -> Self {
    Self { id: val }
  }

  pub fn get_id(&self) -> i32 {
    self.id
  }
}
