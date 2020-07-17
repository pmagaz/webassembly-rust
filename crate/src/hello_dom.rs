//Importamos websys para acceder al DOM
use web_sys::window;

#[wasm_bindgen]
pub fn hello_dom(value: &str) -> Result<(), JsValue> {
  let window = window().expect("Window not found!");
  let document = window.document().expect("Document not found!");
  let body = document.body().expect("Body not found!");
  let div = document.create_element("div")?;
  div.set_inner_html(value);
  body.append_child(&div)?;
  Ok(())
}
