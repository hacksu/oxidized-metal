use wasm_bindgen::prelude::*;
use std::str;

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
    
    let magic_number = parser.read_ascii();
    log(&format!("Magic number: {magic_number}"));
    let max_pixel_value = parser.read_ascii();
    log(&format!("Max pixel value: {max_pixel_value}"));
    let width = parser.read_ascii();
    log(&format!("Width: {width}"));
    let height = parser.read_ascii();
    log(&format!("Height: {height}"));
    let raw = parser.raw();
    log(&format!("Pixel data: {} bytes", raw.len()));
}