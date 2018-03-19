extern crate byteorder;
extern crate crc;
extern crate flate2;

use std::fs::File;
use std::io::Cursor;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;
use self::byteorder::{BigEndian, ReadBytesExt};
use self::crc::crc32;
use self::flate2::Compression;
use self::flate2::write::ZlibEncoder;

const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
const NECESSARY_CHUCKS_NAME: [&str; 4] = ["IHDR", "PLTE", "IDAT", "IEND"];
// const IHDR: [u8; 4] = [73, 72, 68, 82];
// const PLTE: [u8; 4] = [80, 76, 84, 69];
// const IDAT: [u8; 4] = [73, 68, 65, 84];
const IEND: [u8; 4] = [73, 69, 78, 68];

#[derive(Debug)]
pub struct Img<'i> {
  chucks: HashMap<String, Vec<HashMap<&'i str, Vec<u8>>>>,
  meta_data: HashMap<&'i str, u32>,
}

impl<'i> Img<'i> {
  pub fn new(url: &str) -> Img {
    let mut data_buffer: Vec<u8> = Img::get_image_buffer(url).unwrap();
    // 去除 PNG_SIGNATURE
    data_buffer.drain(..8);
    let chucks = Img::chuck_data(&mut data_buffer);
    let meta_data = Img::get_meta_data(chucks.get("IHDR").unwrap());
    Img::gzip(chucks.get("IDAT").unwrap());
    Img { chucks, meta_data }
  }
  pub fn create_png(url: &str, img: &mut Img) -> Result<File> {
    let mut file = File::create(url)?;
    let mut buffer: Vec<u8> = Vec::new();
    buffer.append(&mut PNG_SIGNATURE.to_vec());
    for &key in NECESSARY_CHUCKS_NAME.iter() {
      let _be: bool = img.chucks.contains_key(key);
      if _be {
        for item in img.chucks.get(key).unwrap().iter() {
          buffer.append(&mut item.get("length").unwrap().to_vec());
          buffer.append(&mut item.get("signature").unwrap().to_vec());
          buffer.append(&mut item.get("chuck").unwrap().to_vec());
          buffer.append(&mut item.get("crc").unwrap().to_vec());
        }
      }
    }
    file.write(&buffer)?;
    Ok(file)
  }
  fn get_image_buffer(url: &str) -> Result<Vec<u8>> {
    let mut f = File::open(url)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    return Ok(buffer);
  }
  fn get_meta_data<'d>(ihdr: &Vec<HashMap<&str, Vec<u8>>>) -> HashMap<&'d str, u32> {
    let mut data_buffer = ihdr[0].get("chuck").unwrap().clone();
    let mut meta_data = HashMap::new();
    let width_length = Img::transform_to_decimal(&data_buffer.drain(..4).collect());
    let height_length = Img::transform_to_decimal(&data_buffer.drain(..4).collect());
    let depth: &Vec<u8> = &data_buffer.drain(..1).collect();
    let color_type: Vec<u8> = data_buffer.drain(..1).collect();
    let bpp = Img::get_bpp(3u8);
    let x_comparison = Img::get_xcomparison(&depth[0], bpp);
    meta_data.insert("width", width_length);
    meta_data.insert("height", height_length);
    meta_data.insert("depth", depth[0] as u32);
    meta_data.insert("colorType", color_type[0] as u32);
    meta_data.insert("bpp", bpp);
    meta_data.insert("xComparison", x_comparison);
    return meta_data;
  }
  fn get_bpp(ctype: u8) -> u32 {
    let bpp: u32 = match ctype {
      0 | 3 => 1, // 0：灰度图像  3：索引彩色图像
      2 => 3,     // 彩色图像
      4 => 2,     // 带alpha灰度图像
      6 => 4,     // 带alpha彩色图像
      _ => 3,
    };
    return bpp;
  }
  fn get_xcomparison(depth: &u8, bpp: u32) -> u32 {
    let xcomparison: u32 = match *depth {
      8u8 => bpp,
      16u8 => (bpp * 2),
      _ => bpp,
    };
    return xcomparison;
  }
  fn crc(content: Vec<u8>) -> Vec<u8> {
    let mut _crc = crc32::checksum_ieee(&content);
    return Img::transform_to_vecu8(_crc);
  }
  fn gzip(idat: &Vec<HashMap<&str, Vec<u8>>>) {
    // let mut e = ZlibEncoder::new(idat[0].get("chuck").unwrap(), Compression::default());
    // e.write(b"foo");
    // e.write(b"bar");
    // let compressed_bytes = e.finish();
  }
  // 转换 buffer 为十进制
  fn transform_to_decimal(buffer: &Vec<u8>) -> u32 {
    return Cursor::new(buffer).read_u32::<BigEndian>().unwrap();
  }
  // 转换 integer 为 Vec<u8>
  fn transform_to_vecu8(mut integer: u32) -> Vec<u8> {
    let mut vec = Vec::new();
    while integer > 255 {
      vec.insert(0, (integer % 256) as u8);
      integer = integer / 256;
    }
    vec.insert(0, integer as u8);
    return vec;
  }
  fn chuck_data<'s>(data_buffer: &mut Vec<u8>) -> HashMap<String, Vec<HashMap<&'s str, Vec<u8>>>> {
    let mut buffer = HashMap::new();
    // 获取各个 chuck 块
    while data_buffer.len() > 0 {
      let mut chuck = HashMap::new();
      // chuck 的结构为： length + signature + data + crc
      let mut _data_len = Vec::new();
      let mut _signature = Vec::new();
      let mut _chuck_data = Vec::new();
      let mut _crc = Vec::new();
      // 截取
      _data_len = data_buffer.drain(..4).collect();
      _signature = data_buffer.drain(..4).collect();

      let length = Img::transform_to_decimal(&_data_len);

      _chuck_data = data_buffer.drain(..length as usize).collect();
      _crc = data_buffer.drain(..4).collect();

      chuck.insert("signature", _signature.clone());
      chuck.insert("length", _data_len);
      chuck.insert("chuck", _chuck_data);
      chuck.insert("crc", _crc);

      // 某些 chuck 会重复出现，例如：IDAT
      let str_signature = String::from_utf8(_signature).unwrap();
      let some_u8_value = buffer.get(&str_signature).cloned();
      match some_u8_value {
        Some(_i) => {
          let _v: &mut Vec<HashMap<&str, Vec<u8>>> = buffer.get_mut(&str_signature).unwrap();
          _v.push(chuck);
        }
        None => {
          buffer.insert(str_signature, vec![chuck]);
        }
      }
    }
    return buffer;
  }
}
