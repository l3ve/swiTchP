#![allow(unused)]
use std::fs::File;
use std::{
    collections::HashMap,
    io::{Result, Write},
};
mod data;
mod otree;
mod tools;
// extern crate image;
// 取不到 rgb 可以用，data 受保护
// let mut img = image::open("images/13.jpg").unwrap();
// let mut rgb = img.as_mut_rgb8().unwrap();
// https://www.w3.org/Graphics/GIF/spec-gif89a.txt

const HEADER: [u8; 6] = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61]; //"GIF89a" [71, 73, 70, 56, 57, 97]
const EXTENSION: u8 = 0x21;
const GRAPHIC_CONTROL_EXTENSION: u8 = 0xf9;

const IMAGE_DESCRIPTOR: u8 = 0x2c;
const END: u8 = 0x3b;

pub fn new() {
    // println!("{:?}", data::get());
    let mut color_table = data::get1();
    // println!("{:?}", color_table);
    build_color_table(&mut color_table);
    // let res = create_gif(10, 10);
    // println!("{:?}", res);
}

// 工具类
// 写 gif 图
fn create_gif(w: u16, h: u16) -> Result<()> {
    let global_color_table_flag = false;
    let local_color_table_flag = true;
    let mut f = File::create("l3ve.gif")?;
    let mut data = HEADER.to_vec();
    let mut logical_screen_descriptor =
        build_logical_screen_descriptor(w, h, global_color_table_flag);
    // let mut global_color_table = build_color_table();
    let mut graphic_control_extension = build_graphic_control_extension(10);
    let mut image_descriptor = build_image_descriptor(w, h, local_color_table_flag);
    // let mut local_color_table = build_color_table(vec![]);

    let mut based_image_data = build_based_image_data(
        4,
        tools::lzw_encode(
            &vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            4,
        ),
    );
    let mut based_image_data_2 = build_based_image_data(
        4,
        tools::lzw_encode(
            &vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            4,
        ),
    );
    data.append(&mut logical_screen_descriptor);
    // global_color_table_flag && data.append(&mut global_color_table);
    data.append(&mut graphic_control_extension.clone());
    data.append(&mut image_descriptor.clone());
    // data.append(&mut local_color_table.clone());
    data.append(&mut based_image_data);
    data.append(&mut graphic_control_extension);
    data.append(&mut image_descriptor);
    // data.append(&mut local_color_table);
    data.append(&mut based_image_data_2);
    data.append(&mut vec![END]);
    f.write_all(&data)?;
    f.sync_all()?;
    Ok(())
}
fn build_logical_screen_descriptor(w: u16, h: u16, global_color_table: bool) -> Vec<u8> {
    let mut logical_screen_descriptor = vec![];
    let w = tools::decimal2buffer(w);
    let h = tools::decimal2buffer(h);
    logical_screen_descriptor.append(&mut w.to_vec());
    logical_screen_descriptor.append(&mut h.to_vec());
    // 1.Global Color Table Flag              0 or 1
    // 2.Color Resolution                     3 bits  rgba 8位
    // 3.Sort Flag                            0 or 1
    // 4.Size of Global Color Table           3 bits
    let mut packed_fields: u8 = 0b00000000;
    // 1
    if global_color_table {
        packed_fields = packed_fields | 0b10000000;
    }
    // 2 & 3
    packed_fields = packed_fields | 0b01110000;
    // 4
    if global_color_table {
        packed_fields = packed_fields | 0b00000000;
    }
    logical_screen_descriptor.append(&mut vec![packed_fields]);
    // 背景色在全局颜色列表中的索引，global_color_table 为 false 时，无效
    logical_screen_descriptor.append(&mut vec![0]);
    //  宽高比，一般为 0
    logical_screen_descriptor.append(&mut vec![0]);
    return logical_screen_descriptor;
}
fn build_graphic_control_extension(delay_time: u16) -> Vec<u8> {
    let mut graphic_control_extension = vec![EXTENSION, GRAPHIC_CONTROL_EXTENSION, 0x04];
    // 0.Reserved                    保留 3 bits
    // 1.Disposal Method             3 bits
    //                    0 -   No disposal specified. The decoder is
    //                          not required to take any action.
    //                    1 -   Do not dispose. The graphic is to be left
    //                          in place.
    //                    2 -   Restore to background color. The area used by the
    //                          graphic must be restored to the background color.
    //                    3 -   Restore to previous. The decoder is required to
    //                          restore the area overwritten by the graphic with
    //                          what was there prior to rendering the graphic.
    //                    4-7 - To be defined.
    // 2.User Input Flag             0 or 1
    // 3.Transparency Flag           0 or 1

    //1
    graphic_control_extension.append(&mut vec![0b00000100]);
    // 延时
    let delay_time = tools::decimal2buffer(delay_time);
    graphic_control_extension.append(&mut delay_time.to_vec());
    // 透明索引
    graphic_control_extension.append(&mut vec![0x00]);
    // 终结符
    graphic_control_extension.append(&mut vec![0x00]);
    return graphic_control_extension;
}
fn build_image_descriptor(w: u16, h: u16, local_color_table: bool) -> Vec<u8> {
    let mut image_descriptor = vec![IMAGE_DESCRIPTOR];
    // left top
    image_descriptor.append(&mut vec![0, 0, 0, 0]);
    let mut _w = tools::decimal2buffer(w);
    let mut _h = tools::decimal2buffer(h);
    image_descriptor.append(&mut _w.to_vec());
    image_descriptor.append(&mut _h.to_vec());
    // 1.Local Color Table Flag            0 or 1
    // 2.Interlace Flag                    0 or 1
    // 3.Sort Flag                         0 or 1
    // 4.Reserved                          2 bits
    // 5.Size of Local Color Table         3 bits

    let mut packed_fields: u8 = 0b00000000;
    // 1
    if local_color_table {
        packed_fields = packed_fields | 0b10000000;
    }
    // 2 & 3 & 4
    packed_fields = packed_fields | 0b00000000;
    // 5
    if local_color_table {
        packed_fields = packed_fields | 0b00000000;
    }
    image_descriptor.append(&mut vec![packed_fields]);
    return image_descriptor;
}
fn build_based_image_data(lzw: u8, mut data: Vec<u8>) -> Vec<u8> {
    let mut based_image_data = vec![lzw];
    based_image_data.append(&mut vec![data.len() as u8]);
    based_image_data.append(&mut data);
    based_image_data.append(&mut vec![0]);
    return based_image_data;
}
fn build_color_table(data: &mut Vec<u8>) -> Vec<[u8; 8]> {
    // HashMap 是为了优化效率
    let mut rgb_to_oct: HashMap<&[u8], [u8; 8]> = HashMap::new();
    let mut binary_rgb_table: Vec<[u8; 8]> = vec![];
    let mut otree = otree::OTree::new();

    if data.len() % 3 != 0 {
        return binary_rgb_table;
    }

    for rgb in data.chunks(3) {

        match rgb_to_oct.get(&rgb) {
            Some(binary_color) => {
                binary_rgb_table.push(binary_color.clone());
                otree.insert(binary_color);
            }
            None => {
                // 获取二进制色彩值
                let binary_color = tools::rgb2binary(rgb);
                rgb_to_oct.insert(rgb, binary_color);
                binary_rgb_table.push(binary_color);
                otree.insert(&binary_color);
            }
        }
    }
    println!("rgb_to_oct---{:?}", rgb_to_oct);
    println!("otree---{:?}", otree);
    otree.brief();
    // println!("{:?}", binary_rgb_table);
    return binary_rgb_table;
}
