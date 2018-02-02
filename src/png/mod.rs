use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const IHDR: [u8; 4] = [73, 72, 68, 82];
const PLTE: [u8; 4] = [80, 76, 84, 69];
const IDAT: [u8; 4] = [73, 68, 65, 84];
const IEND: [u8; 4] = [73, 69, 78, 68];


#[derive(Debug)]
pub struct Img<'i> {
  signature: Vec<u8>,
  ihdr: HashMap<&'i str, Vec<u8>>,
  plte: HashMap<&'i str, Vec<u8>>,
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
      plte: Img::chuck_data(&mut data_buffer, PLTE.to_vec()),
      idat: Img::chuck_data(&mut data_buffer, IDAT.to_vec()),
      iend: Img::chuck_data(&mut data_buffer, IEND.to_vec()),
      // data_buffer: data_buffer,
    }
  }
  pub fn write(url: &str, img: &mut Img) -> Result<File> {
    let mut file = File::create(url)?;
    let mut buffer: Vec<u8> = Vec::new();
    buffer.append(&mut img.signature);
    let mut ihdr_length = img.ihdr.get("length").unwrap().clone();
    let mut ihdr_signature = img.ihdr.get("signature").unwrap().clone();
    let mut ihdr_data = img.ihdr.get("chuck").unwrap().clone();
    let mut ihdr_crc = img.ihdr.get("crc").unwrap().clone();

    let mut plte_length = img.plte.get("length").unwrap().clone();
    let mut plte_signature = img.plte.get("signature").unwrap().clone();
    let mut plte_data = img.plte.get("chuck").unwrap().clone();
    let mut plte_crc = img.plte.get("crc").unwrap().clone();

    let mut idat_length = img.idat.get("length").unwrap().clone();
    let mut idat_signature = img.idat.get("signature").unwrap().clone();
    let mut idat_data = img.idat.get("chuck").unwrap().clone();
    let mut idat_crc = img.idat.get("crc").unwrap().clone();

    let mut iend_length = img.iend.get("length").unwrap().clone();
    let mut iend_signature = img.iend.get("signature").unwrap().clone();
    let mut iend_data = img.iend.get("chuck").unwrap().clone();
    let mut iend_crc = img.iend.get("crc").unwrap().clone();

    buffer.append(&mut ihdr_length);
    buffer.append(&mut ihdr_signature);
    buffer.append(&mut ihdr_data);
    buffer.append(&mut ihdr_crc);
    buffer.append(&mut plte_length);
    buffer.append(&mut plte_signature);
    buffer.append(&mut plte_data);
    buffer.append(&mut plte_crc);
    buffer.append(&mut idat_length);
    buffer.append(&mut idat_signature);
    buffer.append(&mut idat_data);
    buffer.append(&mut idat_crc);
    buffer.append(&mut iend_length);
    buffer.append(&mut iend_signature);
    buffer.append(&mut iend_data);
    buffer.append(&mut iend_crc);

    file.write(&buffer)?;
    Ok(file)
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
    let mut signature = Vec::new();
    let mut data_len = Vec::new();
    let mut chuck_data = Vec::new();
    let mut crc = Vec::new();
    if position.0 == position.1 {
      signature = vec![];
      data_len = vec![];
      chuck_data = vec![];
      crc = vec![];
    } else {
      signature = data_buffer.drain(position.0..position.1).collect();
      data_len = data_buffer.drain(position.0 - 4..position.0).collect();
      // 转换 length 为十进制
      let mut length = 0u32;
      for (i, &item) in data_len.iter().enumerate() {
        if i == 3 {
          length = length + item as u32;
        } else {
          length = length + item as u32 * ((3 - i) * 255) as u32;
        }
      }
      chuck_data = data_buffer
        .drain(position.0 - 4..position.0 - 4 + length as usize)
        .collect();
      crc = data_buffer.drain(position.0 - 4..position.0).collect();
    }
    (signature, chuck_data, data_len, crc)
  }
}
