use byteorder::{LittleEndian, ReadBytesExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub intensity: u16,
    pub classification: u8,
}

impl Point {
    pub fn default() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            intensity: 0,
            classification: 0,
        }
    }

    pub fn from(mut buffer: &[u8]) -> Point {
        let x = f64::from(buffer.read_u32::<LittleEndian>().unwrap());
        let y = f64::from(buffer.read_u32::<LittleEndian>().unwrap());
        let z = f64::from(buffer.read_u32::<LittleEndian>().unwrap());
        let intensity = buffer.read_u16::<LittleEndian>().unwrap();

        // Currently we skip rnse bitfield
        buffer.read_u8().unwrap();

        let csup_bitfield = buffer.read_u8().unwrap();

        Point {
            x,
            y,
            z,
            intensity,
            classification: csup_bitfield & 0x1f,
        }
    }
}
