extern crate byteorder;
extern crate crc;
extern crate flate2;

use std::fs::File;
use std::io::Cursor;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;
use self::byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Img<'i> {
  chucks: HashMap<String, Vec<HashMap<&'i str, Vec<u8>>>>,
  meta_data: HashMap<&'i str, u32>,
}

impl<'i> Img<'i> {
  pub fn new(url: &str) {
    let mut data_buffer: Vec<u8> = Img::get_image_buffer(url).unwrap();
    let chucks = Img::chuck_data(&mut data_buffer);
    println!("{:?}", chucks);
  }
  fn get_image_buffer(url: &str) -> Result<Vec<u8>> {
    let mut f = File::open(url)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    return Ok(buffer);
  }
  fn chuck_data(data: &mut Vec<u8>) -> HashMap<Vec<u8>, [Vec<u8>; 2]> {
    let mut buffer = HashMap::new();
    data.drain(..2);
    while data.len() > 2 {
      // chuck 的结构为： length  + data
      let mut _data_len = Vec::new();
      let mut _signature = Vec::new();
      let mut _chuck_data = Vec::new();

      if data[0] == 255u8 {
        _signature = data.drain(..2).collect();
        _data_len = data.drain(..2).collect();
        _chuck_data = data
          .drain(..(Img::transform_to_decimal(&_data_len) - 2) as usize)
          .collect();
      } else {
        _data_len = data.drain(2..).collect();
        _chuck_data = data.clone();
        _signature = vec![1,2,3];
      }

      buffer.insert(_signature, [_data_len, _chuck_data]);
    }
    return buffer;
  }
  // 转换 buffer 为十进制
  fn transform_to_decimal(buffer: &Vec<u8>) -> u32 {
    let mut data = vec![0,0];
    data.append(&mut buffer.clone());
    return Cursor::new(data).read_u32::<BigEndian>().unwrap();
  }
}
