use byteorder::{LittleEndian, ReadBytesExt};
use laz::las::file::QuickHeader;
use std::io::{BufReader, Cursor, Seek, SeekFrom};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy)]
pub struct Header {
    pub major: u8,
    pub minor: u8,
    pub offset_to_points: u32,
    pub num_vlrs: u32,
    pub point_format_id: u8,
    pub point_size: u16,
    pub num_points: u64,
    pub header_size: u16,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
}

impl Header {
    pub fn default() -> Header {
        Header {
            major: 0,
            minor: 0,
            offset_to_points: 0,
            num_vlrs: 0,
            point_format_id: 0,
            point_size: 0,
            num_points: 0,
            header_size: 0,
            scale_x: 0.0,
            scale_y: 0.0,
            scale_z: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            max_z: 0.0,
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
        }
    }

    pub fn from(mut buffer: BufReader<Cursor<Vec<u8>>>) -> Header {
        let qh = QuickHeader::read_from(&mut buffer).unwrap();

        buffer.seek(SeekFrom::Start(131)).expect("TODO: panic message");

        Header {
            major: qh.major,
            minor: qh.minor,
            offset_to_points: qh.offset_to_points,
            num_vlrs: qh.num_vlrs,
            point_format_id: qh.point_format_id,
            point_size: qh.point_size,
            num_points: qh.num_points,
            header_size: qh.header_size,
            scale_x: buffer.read_f64::<LittleEndian>().unwrap(),
            scale_y: buffer.read_f64::<LittleEndian>().unwrap(),
            scale_z: buffer.read_f64::<LittleEndian>().unwrap(),
            offset_x: buffer.read_f64::<LittleEndian>().unwrap(),
            offset_y: buffer.read_f64::<LittleEndian>().unwrap(),
            offset_z: buffer.read_f64::<LittleEndian>().unwrap(),
            max_x: buffer.read_f64::<LittleEndian>().unwrap(),
            max_y: buffer.read_f64::<LittleEndian>().unwrap(),
            max_z: buffer.read_f64::<LittleEndian>().unwrap(),
            min_x: buffer.read_f64::<LittleEndian>().unwrap(),
            min_y: buffer.read_f64::<LittleEndian>().unwrap(),
            min_z: buffer.read_f64::<LittleEndian>().unwrap(),
        }
    }
}

impl Clone for Header {
    fn clone(&self) -> Self {
        Header {
            major: self.major,
            minor: self.minor,
            offset_to_points: self.offset_to_points,
            num_vlrs: self.num_vlrs,
            point_format_id: self.point_format_id,
            point_size: self.point_size,
            num_points: self.num_points,
            header_size: self.header_size,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
            scale_z: self.scale_z,
            offset_x: self.offset_x,
            offset_y: self.offset_y,
            offset_z: self.offset_z,
            max_x: self.max_x,
            max_y: self.max_y,
            max_z: self.max_z,
            min_x: self.min_x,
            min_y: self.min_y,
            min_z: self.min_z,
        }
    }
}
