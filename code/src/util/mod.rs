use std::{fs, str};
use std::fs::File;
use crate::rendering::render::Vertex;

pub mod input_handler;
pub mod ray_library;

pub fn read_shader(buf: &[u8]) -> &str{

    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    return s;
}
