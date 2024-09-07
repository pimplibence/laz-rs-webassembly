use crate::{header, point};
use js_sys::Uint8Array;
use laz::las::file::SimpleReader;
use std::io::{BufReader, Cursor};

pub struct Reader {}

impl Reader {
    pub fn read_header(data: Uint8Array) -> header::Header {
        let vec = data.to_vec();
        let cursor = Cursor::new(vec);
        let buffer = BufReader::new(cursor);

        header::Header::from(buffer)
    }

    pub fn read_points(data: Uint8Array) -> Vec<point::Point> {
        let vec = data.to_vec();
        let cursor = Cursor::new(vec);
        let buffer = BufReader::new(cursor);

        let mut sr = SimpleReader::new(buffer).unwrap();

        let mut result: Vec<point::Point> = Vec::default();

        while let Some(point) = sr.read_next() {
            let p = point::Point::from(point.unwrap());
            result.push(p)
        }

        result
    }
}