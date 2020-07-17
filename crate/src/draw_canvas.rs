use web_sys::window;

#[wasm_bindgen]
pub fn draw_canvas() {
  //Accedemos a Document
  let document = web_sys::window().unwrap().document().unwrap();
  //Obtenemos la instancia del Canvas
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

  //Otenemos el contexto, en este caso 2D
  let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();

  //Comienza una nueva ruta
  context.begin_path();

  //Elegiemos el color del cuadrado
  context.set_fill_style(&"rgb(130,150,30)".into());

  //Rellenamos un cuadrado de 100 x 100
  context.fill_rect(0.0, 0.0, 100.0, 100.0);

  context.stroke();
}
