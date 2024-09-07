use crate::{header, point};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Result {
    pub header: header::Header,
    pub coordinates_length: usize,
    pub coordinates_pointer: *const f32,
    pub intensity_length: usize,
    pub intensity_pointer: *const u16,
    pub classification_length: usize,
    pub classification_pointer: *const u8,
}

#[wasm_bindgen]
impl Result {
    pub fn new() -> Result {
        Result {
            header: header::Header::default(),
            coordinates_length: 0,
            coordinates_pointer: Vec::default().as_ptr(),
            intensity_length: 0,
            intensity_pointer: Vec::default().as_ptr(),
            classification_length: 0,
            classification_pointer: Vec::default().as_ptr(),
        }
    }

    pub fn set_header(&mut self, header: header::Header) {
        self.header = header;
    }

    pub fn set_points(&mut self, points: Vec<point::Point>) {
        let mut points_vec: Vec<f32> = Vec::new();
        let mut intensity_vec: Vec<u16> = Vec::new();
        let mut classification_vec: Vec<u8> = Vec::new();

        let scale_x = self.header.scale_x as f32;
        let scale_y = self.header.scale_y as f32;
        let scale_z = self.header.scale_z as f32;

        for point in &points {
            points_vec.push(point.x as f32 * scale_x);
            points_vec.push(point.y as f32 * scale_y);
            points_vec.push(point.z as f32 * scale_z);

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
