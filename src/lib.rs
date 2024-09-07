use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

mod header;
mod point;
mod reader;
mod result;

#[wasm_bindgen]
pub fn read_sync(data: Uint8Array) -> result::Result {
    let d1 = data.clone();
    let d2 = data.clone();

    let header = reader::Reader::read_header(d1);
    let points = reader::Reader::read_points(d2);

    let mut result = result::Result::new();

    result.set_header(header);
    result.set_points(points);

    result
}