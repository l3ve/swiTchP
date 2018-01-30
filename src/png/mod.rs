use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const IHDR: [u8; 4] = [73, 72, 68, 82];
const IDAT: [u8; 4] = [73, 68, 65, 84];
const IEND: [u8; 4] = [73, 69, 78, 68];


#[derive(Debug)]
pub struct Img<'i> {
  signature: Vec<u8>,
  ihdr: HashMap<&'i str, Vec<u8>>,
  idat: HashMap<&'i str, Vec<u8>>,
  iend: HashMap<&'i str, Vec<u8>>,
  // data_buffer: Vec<u8>,
}

impl<'i> Img<'i> {
  pub fn new(url: &str) -> Img {
    let mut data_buffer: Vec<u8> = Img::get_image_buffer(url).unwrap();
    data_buffer.drain(..8);
    Img {
      signature: PNG_SIGNATURE.to_vec(),
      ihdr: Img::chuck_data(&mut data_buffer, IHDR.to_vec()),
      idat: Img::chuck_data(&mut data_buffer, IDAT.to_vec()),
      iend: Img::chuck_data(&mut data_buffer, IEND.to_vec()),
      // data_buffer: data_buffer,
    }
  }
  fn get_image_buffer(url: &str) -> Result<Vec<u8>> {
    let mut f = File::open(url)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
  }
  fn chuck_data<'a>(
    data_buffer: &mut Vec<u8>,
    chuck_signature: Vec<u8>,
  ) -> HashMap<&'a str, Vec<u8>> {
    let mut chuck = HashMap::new();
    let (signature, chuck_data, data_len, crc) = Img::split_data(data_buffer, chuck_signature);
    chuck.insert("signature", signature);
    chuck.insert("length", data_len);
    chuck.insert("chuck", chuck_data);
    chuck.insert("crc", crc);
    chuck
  }
  fn split_data(
    data_buffer: &mut Vec<u8>,
    signature: Vec<u8>,
  ) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut position = (0, 0);
    let mut i = 0;
    let mut stop_loop = false;
    // 找出标识符在数据中的位置
    while i < data_buffer.len() {
      let mut ii = 0;
      let mut _i = i;
      while ii < signature.len() {
        if data_buffer[_i] == signature[ii] {
          if ii + 1 == signature.len() {
            // +1 是为了保证 drain 时，截取到末尾
            position = (i, i + ii + 1);
            stop_loop = true;
          }
          ii = ii + 1;
          _i = _i + 1;
          if _i >= data_buffer.len() {
            break;
          }
        } else {
          break;
        }
      }
      if stop_loop {
        break;
      }
      i = i + 1;
    }
    // chuck 的结构为： length + signature + data + crc
    let signature: Vec<u8> = data_buffer.drain(position.0..position.1).collect();
    let data_len: Vec<u8> = data_buffer.drain(position.0 - 4..position.0).collect();
    // 转换 length 为十进制
    let mut length = 0u8;
    // data_len.reverse();
    for (i, &item) in data_len.iter().enumerate() {
      if (i == 3) {
        length = length + &item;
      } else {
        length = length + &item * (i + 3 * 255) as u8;
      }
    }
    let chuck_data: Vec<u8> = data_buffer
      .drain(position.0 - 4..position.0 - 4 + length as usize)
      .collect();
    let crc: Vec<u8> = data_buffer.drain(position.0 - 4..position.0).collect();
    (signature, chuck_data, data_len, crc)
  }
}
