//crate para serializar / deserializar
use serde::{Deserialize, Serialize};
// JsCast permite devolver objetos JavaScript desde Rust
use wasm_bindgen::JsCast;
//JsFuture transforma un Future de Rust en una Promesa
use wasm_bindgen_futures::JsFuture;
// Dependencias de websys
use web_sys::{window, Request, RequestInit, RequestMode, Response};

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
// Función async, igual que en JS que devuelve un JsValue (valor JavaScript)
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
