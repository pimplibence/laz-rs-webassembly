use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

mod reader;

#[wasm_bindgen]
pub struct Result {
    pub header: reader::header::Header,
    pub coordinates_length: usize,
    pub coordinates_pointer: *const f64,
    pub intensity_length: usize,
    pub intensity_pointer: *const u16,
    pub classification_length: usize,
    pub classification_pointer: *const u8,
}

#[wasm_bindgen]
impl Result {
    fn new() -> Result {
        Result {
            header: reader::header::Header::default(),
            coordinates_length: 0,
            coordinates_pointer: Vec::default().as_ptr(),
            intensity_length: 0,
            intensity_pointer: Vec::default().as_ptr(),
            classification_length: 0,
            classification_pointer: Vec::default().as_ptr(),
        }
    }

    fn set_header(&mut self, header: reader::header::Header) {
        self.header = header;
    }

    fn set_points(&mut self, points: Vec<reader::point::Point>) {
        let mut points_vec: Vec<f64> = Vec::new();
        let mut intensity_vec: Vec<u16> = Vec::new();
        let mut classification_vec: Vec<u8> = Vec::new();

        for point in &points {
            points_vec.push(point.x * self.header.scale_x + self.header.offset_x);
            points_vec.push(point.y * self.header.scale_y + self.header.offset_y);
            points_vec.push(point.z * self.header.scale_z + self.header.offset_z);
            intensity_vec.push(point.intensity);
            classification_vec.push(point.classification);
        }

        self.coordinates_length = points_vec.len();
        self.coordinates_pointer = points_vec.leak().as_ptr();
        self.intensity_length = intensity_vec.len();
        self.intensity_pointer = intensity_vec.leak().as_ptr();
        self.classification_length = classification_vec.len();
        self.classification_pointer = classification_vec.leak().as_ptr();
    }
}

#[wasm_bindgen]
pub fn read_sync(data: Uint8Array) -> Result {
    let d1 = data.clone();
    let d2 = data.clone();

    let header = reader::Reader::read_header(d1);
    let points = reader::Reader::read_points(d2);

    let mut result = Result::new();

    result.set_header(header);
    result.set_points(points);

    result
}
