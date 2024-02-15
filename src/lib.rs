use wasm_bindgen::prelude::*;

// exposing the console.log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(js_name = processImage)]
pub fn process_image(data: Vec<u8>) {
   let result = image::load_from_memory_with_format(&data, image::ImageFormat::Png);
   match result {
      Ok(image) => {
         log(image.height().to_string().as_str());
         log(image.width().to_string().as_str());
      },
      Err(err) => {
         log(err.to_string().as_str());
      }
   }
}