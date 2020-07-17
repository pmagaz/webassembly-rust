use wasm_bindgen::prelude::*;
// Importamos console del crate web_sys
//use web_sys::console::log;
use serde::{Deserialize, Serialize};
use std::f64;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use web_sys::window;

#[wasm_bindgen]
pub fn multiply(a: isize, b: isize) -> isize {
  a * b
}

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

#[wasm_bindgen]
pub fn canvas_draw() {
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

//Struct para el Post
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  pub id: String,
  pub title: String,
}
//Array de posts
#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
  pub posts: Vec<Post>,
}

#[wasm_bindgen]
// Función async, igual que en JS que devuelve un JsValue
pub async fn load_posts() -> Result<JsValue, JsValue> {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  //Preparamos la request
  let request = Request::new_with_str_and_init("https://pablomagaz.com/api/posts", &opts)?;

  let window = web_sys::window().unwrap();
  //Ejecutamos la request eseperando recibir un Future / promesa
  //El .await es el equivcalente
  let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
  let resp: Response = resp_value.dyn_into().unwrap();
  //Pasamos la respuesta a json
  let json = JsFuture::from(resp.json()?).await?;
  //Serializamos el Json a structs de Rust (Posts) esto nos permitiria
  //manejar los datos como structs nativos de Rust.
  let posts: Posts = json.into_serde().unwrap();
  //... calculos pesados
  //Pasamos nuestro struct de Posts a objetos JavaScript
  Ok(JsValue::from_serde(&posts).unwrap())
}
