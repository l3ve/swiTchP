use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use std::collections::HashMap;

pub const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];


#[derive(Debug)]
pub struct Img {
  signature: Vec<u8>,
  ihdr: HashMap<char, Vec<u8>>,
  idat: HashMap<char, Vec<u8>>,
  iend: HashMap<char, Vec<u8>>,
  data_buffer: Vec<u8>,
}

impl Img {
  pub fn new(url: &str) -> Img {
    let png_signature: Vec<u8> = PNG_SIGNATURE.to_vec();
    let mut data_buffer: Vec<u8> = Img::get_image_buffer(url).unwrap();
    let sdf = Img::split_data(&mut data_buffer, &png_signature, true);
    println!("{:?}======={:?}", sdf, data_buffer);
    Img {
      signature: png_signature,
      ihdr: Img::chuck_data(&mut data_buffer),
      idat: HashMap::new(),
      iend: HashMap::new(),
      data_buffer: data_buffer,
    }
  }
  fn get_image_buffer(url: &str) -> Result<Vec<u8>> {
    let mut f = File::open(url)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
  }
  fn chuck_data(data_buffer:&mut Vec<u8>) -> HashMap<char, Vec<u8>> {
    let mut chuck = HashMap::new();
    chuck
  }
  fn split_data(a: &mut Vec<u8>, b: &Vec<u8>, first_one: bool) -> Vec<u8> {
    let mut position = (0, 0);
    let mut i = 0;
    let mut stop_loop = false;
    while i < a.len() {
      let mut ii = 0;
      let mut _i = i;
      while ii < b.len() {
        if a[_i] == b[ii] {
          if ii + 1 == b.len() {
            // +1 是为了保证 drain 时，截取到末尾
            position = (i, i + ii + 1);
            stop_loop = true;
          }
          ii = ii + 1;
          _i = _i + 1;
          if _i >= a.len() {
            break;
          }
        } else {
          break;
        }
      }
      if stop_loop && first_one {
        break;
      }
      i = i + 1;
    }
    let u: Vec<u8> = a.drain(position.0..position.1).collect();
    u
  }
}
