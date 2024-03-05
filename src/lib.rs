use std::str;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

// exposing the console.log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Parser {
    data: Vec<u8>,
    index: usize,
}

impl Parser {
    fn next(&mut self) {
        while self.data[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
    }

    fn read_ascii(&mut self) -> &str {
        self.next();
        let initial_index = self.index;
        while !self.data[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
        return str::from_utf8(&self.data[initial_index..self.index]).unwrap();
    }

    fn raw(&mut self) -> Vec<u8> {
        self.next();
        return self.data[self.index..].to_vec();
    }
}

#[wasm_bindgen(js_name = processImage)]
pub fn process_image(data: Vec<u8>) {
    let mut parser = Parser {
        data: data,
        index: 0,
    };

    let _magic_number = parser.read_ascii();
    let width: u32 = parser.read_ascii().parse().expect("Failed to convert width string to u32.");
    let height: u32 = parser.read_ascii().parse().expect("Failed to convert height string to u32.");
    let _max_pixel_value = parser.read_ascii();
    let raw = parser.raw();

    // convert our RGB image to an RBGA image, but it's greyscale
    let mut output: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < (width * height * 3) as usize {
        let r = raw[index] as u32;
        let g = raw[index + 1] as u32;
        let b = raw[index + 2] as u32;
        let avg = (r + g + b) / 3;
        
        for _ in 0..3 {
            output.push(avg as u8)
        }
        output.push(u8::MAX);

        index += 3;
    }

    // create a new canvas element
    let window = web_sys::window().unwrap();    
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.set_attribute("width", width.to_string().as_str()).unwrap();
    canvas.set_attribute("height", height.to_string().as_str()).unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("Failed to cast Element to CanvasElement");

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Failed to grab 2d canvas context.");
    
    // put out image data in the canvas element
    let image = ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&output), width, height).expect("Failed to create image data.");
    context.put_image_data(&image, 0.0, 0.0).expect("Failed to put image data on canvas.");

    // add the canvas element to the DOM!
    body.append_child(&canvas).unwrap();
}
