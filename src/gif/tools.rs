use byteorder::{ByteOrder, LittleEndian};
use lzw::{Decoder, Encoder, LsbReader, LsbWriter};
// use std::fs::File;
// use std::io::{ Result};

pub fn rgb2binary(rgb: &[u8]) -> [u8; 8] {
  let r = rgb[0];
  let g = rgb[1];
  let b = rgb[2];
  let mut binary = [0u8; 8];
  for i in 0..8 {
    let i = 7 - i;
    let r1 = r << i >> 7 << 2;
    let g1 = g << i >> 7 << 1;
    let b1 = b << i >> 7;
    binary[i] = r1 | g1 | b1;
  }
  return binary;
}
pub fn binary2rgb(binary: &[u8]) -> [u8; 3] {
  let mut rgb = [0u8; 3];
  for (i, val) in binary.iter().enumerate() {
    let ii = 7 - i;
    rgb[0] = rgb[0] | val >> 2 << ii;
    rgb[1] = rgb[1] | val << 6 >> 7 << ii;
    rgb[2] = rgb[2] | val << 7 >> 7 << ii;
  }
  return rgb;
}

pub fn decimal2buffer(target: u16) -> [u8; 2] {
  let mut buf = [0, 0];
  LittleEndian::write_u16(&mut buf, target);
  return buf;
}

pub fn _log2(i: u8) -> u8 {
  let mut times = 0u32;
  loop {
    if i == 2u8.pow(times) {
      break;
    }
    times = times + 1;
  }
  return times as u8;
}

pub fn _lzw_decode(_data: &Vec<u8>, size: u8) -> Vec<u8> {
  let mut data = _data.clone();
  let mut decoder = Decoder::new(LsbReader::new(), size);
  let mut res = vec![];
  while data.len() > 0 {
    let (start, bytes) = decoder.decode_bytes(&data).unwrap();
    data = data[start..].to_vec();
    res.extend(bytes.iter().map(|&i| i));
  }
  return res;
}

pub fn lzw_encode(_data: &Vec<u8>, size: u8) -> Vec<u8> {
  let data = _data.clone();
  let mut res = vec![];
  {
    let mut enc = Encoder::new(LsbWriter::new(&mut res), size).unwrap();
    enc.encode_bytes(&data).unwrap();
  }
  return res;
}
