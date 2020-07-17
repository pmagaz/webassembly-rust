import "core-js/stable";
import "regenerator-runtime/runtime";

import { multiply } from '../crate/src/multiply.rs';
import { RustStruct } from '../crate/src/rust_struct.rs';
import { hello_dom } from '../crate/src/hello_dom.rs';
import { canvas_draw } from '../crate/src/draw_canvas.rs';
import { load_posts } from '../crate/src/load_posts.rs';

let result = multiply(10, 10);
console.log(result);

const rustStruct = new RustStruct(12345);
const id = rustStruct.get_id();
console.log(id);
hello_dom("Hello DOM!");
canvas_draw();

const getPosts = async () => {
  let posts = await load_posts();
  return posts;
};

getPosts();
