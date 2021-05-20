
#[derive(Debug)]
struct Gif {
    header: Vec<u8>,
    width: u16,
    height: u16,
    table: Vec<u8>,
    data_source: Vec<u8>,
    global_color_table: Vec<u8>,
    base_image_data: Vec<Vec<u8>>,
    buffer: Vec<u8>,
}

impl Gif {
    fn new(mut data: Vec<u8>) -> Gif {
        let _header: Vec<u8> = data.drain(..6).collect();
        Gif {
            header: _header,
            width: 0,
            height: 0,
            data_source: data.clone(),
            table: vec![],
            global_color_table: vec![],
            base_image_data: vec![],
            buffer: data,
        }
    }
    fn analysis(&mut self) {
        let (flag, size) = self.get_logical_screen_descriptor();
        if flag {
            self.get_global_color_table(size);
        }
        self.chuck_data();
    }
    fn get_logical_screen_descriptor(&mut self) -> (bool, usize) {
        println!("logical screen descriptor");
        let mut data: Vec<u8> = self.buffer.drain(..7).collect();
        // 高宽
        let width = buffer_to_decimal(&data.drain(..2).collect());
        let height = buffer_to_decimal(&data.drain(..2).collect());
        let compression_bit: Vec<u8> = data.drain(..1).collect();
        let mut compression_bit = decimal_to_binary(compression_bit[0]);
        // 是否存在全局颜色
        let global_color_table_flag: String = compression_bit.drain(..1).collect();
        let global_color_table_flag = string_to_decimal(&global_color_table_flag);
        let global_color_table_flag: bool = number_to_bool(global_color_table_flag);
        // 颜色的位数
        let _color_resolution: String = compression_bit.drain(..3).collect();
        let _color_resolution = binary_to_decimal(_color_resolution) + 1;
        // 颜色列表排序方式
        // 0 – 没有排序过
        // 1 – 递减排序，最重要的颜色优先
        let _sort_flag: String = compression_bit.drain(..1).collect();
        let _sort_flag = string_to_decimal(&_sort_flag);
        // 全局颜色的大小
        let global_color_table_size: String = compression_bit.drain(..3).collect();
        let global_color_table_size =
            2_usize.pow((binary_to_decimal(global_color_table_size) + 1) as u32);
        // 背景颜色在全局颜色里的索引
        let _background_color_index: Vec<u8> = data.drain(..1).collect();
        // 宽高比
        let _aspect_ratio: Vec<u8> = data.drain(..1).collect();
        let _aspect_ratio: bool = number_to_bool(_aspect_ratio[0]);
        self.width = width;
        self.height = height;
        return (global_color_table_flag, global_color_table_size);
    }
    fn get_global_color_table(&mut self, size: usize) {
        println!("global color table");
        let _size = (size * 3) as usize;
        let global_color_table: Vec<u8> = self.buffer.drain(.._size).collect();
        self.global_color_table = global_color_table;
    }
    fn get_local_color_table(&mut self, size: usize) {
        println!("local color table");
        let _size = (size * 3) as usize;
        let _local_color_table: Vec<u8> = self.buffer.drain(.._size).collect();
    }
    fn get_based_image_data(&mut self) {
        let _lzw_minimum_color_size: Vec<u8> = self.buffer.drain(..1).collect();
        let mut len: Vec<u8> = self.buffer.drain(..1).collect();
        let mut base_image_data_index = vec![];
        while len[0] != 0 {
            // println!("{:?}", len);
            // if (self.buffer.len() < 700) {
            //     println!("{:?}", self.buffer);
            //     println!("buffer-len:{:?}", self.buffer.len());
            // }
            let mut data: Vec<u8> = self.buffer.drain(..(len[0] as usize)).collect();
            base_image_data_index.append(&mut data);
            len = self.buffer.drain(..1).collect();
        }
        base_image_data_index = lzw_decode(&base_image_data_index, _lzw_minimum_color_size[0]);
        println!("lzw end");
        self.base_image_data.push(base_image_data_index);
    }
    fn chuck_data(&mut self) {
        while self.buffer.len() > 0 {
            let sign: Vec<u8> = self.buffer.drain(..1).collect();
            match sign[0] {
                EXTENSION => self.chuck_extension(),
                IMAGE_DESCRIPTOR => self.chuck_id(),
                END => println!("This is the end~"),
                _ => self.no_match(sign[0]),
            }
        }
    }
    fn chuck_extension(&mut self) {
        let sign: Vec<u8> = self.buffer.drain(..1).collect();
        match sign[0] {
            APPLICATION_EXTENSION => self.chuck_ae(),
            GRAPHIC_CONTROL_EXTENSION => self.chuck_gce(),
            COMMENT_EXTENSION => self.chuck_ce(),
            PLAIN_TEXT_EXTENSION => self.chuck_pte(),
            _ => self.no_match(sign[0]),
        }
    }
    fn no_match(&mut self, no_match: u8) {
        println!("no_match_sign:{:?}", no_match);
    }
    fn chuck_ae(&mut self) {
        println!("Application Extension");
        let mut length: Vec<u8> = self.buffer.drain(..1).collect();
        let _ae_netscape: Vec<u8> = self.buffer.drain(..(length[0] as usize)).collect();
        length = self.buffer.drain(..1).collect();
        let _ae_data: Vec<u8> = self.buffer.drain(..(length[0] as usize)).collect();
        let _block_terminator: Vec<u8> = self.buffer.drain(..1).collect();
    }
    fn chuck_ce(&mut self) {
        println!("Comment Extension");
        let length: Vec<u8> = self.buffer.drain(..1).collect();
        let _ce_data: Vec<u8> = self.buffer.drain(..(length[0] as usize)).collect();
        let _block_terminator: Vec<u8> = self.buffer.drain(..1).collect();
    }
    fn chuck_pte(&mut self) {
        println!("Plain Text Extension");
        let mut length: Vec<u8> = self.buffer.drain(..1).collect();
        let _pte: Vec<u8> = self.buffer.drain(..(length[0] as usize)).collect();
        length = self.buffer.drain(..1).collect();
        let _pt_data: Vec<u8> = self.buffer.drain(..(length[0] as usize)).collect();
        let _block_terminator: Vec<u8> = self.buffer.drain(..1).collect();
    }
    fn chuck_gce(&mut self) {
        println!("Graphic Control Extension");
        let length: Vec<u8> = self.buffer.drain(..1).collect();
        let mut gce_data: Vec<u8> = self.buffer.drain(..(length[0] as usize)).collect();
        let _block_terminator: Vec<u8> = self.buffer.drain(..1).collect();
        let compression_bit: Vec<u8> = gce_data.drain(..1).collect();
        let compression_bit: String = decimal_to_binary(compression_bit[0]);
        let mut compression_bit: String = pad_start(compression_bit);
        // 前三位 bit 保留,前置3位，转换是去掉了
        let _reserved: String = compression_bit.drain(..3).collect();
        // disposal_method_values :
        // 0 – 未指定，解码器不需要做任何动作，这个选项可以将一个全尺寸，非透明框架替换为另一个。
        // 1 – 不要处置，在此选项中，未被下一帧覆盖的任何像素继续显示。
        // 2 – 还原为背景图像，被图像使用的区域必须还原为背景颜色。
        // 3 – 还原为上一个，解码器必须还原被图像覆盖的区域为之前渲染的图像。
        // 4-7 – 未定义。
        let _disposal_method: String = compression_bit.drain(..3).collect();
        let _disposal_method = binary_to_decimal(_disposal_method);
        let _user_input_flag: String = compression_bit.drain(..1).collect();
        let transparent_color_flag: String = compression_bit.drain(..1).collect();
        let transparent_color_flag: bool =
            number_to_bool(string_to_decimal(&transparent_color_flag));
        // 延迟时长
        let _delay_time = buffer_to_decimal(&gce_data.drain(..2).collect());
        // 透明色索引
        let mut _transparent_color_index = 0;
        if transparent_color_flag {
            _transparent_color_index = gce_data[0];
        }
    }
    fn chuck_id(&mut self) {
        println!("Image Descriptor");
        let mut data: Vec<u8> = self.buffer.drain(..9).collect();
        let _left = buffer_to_decimal(&data.drain(..2).collect());
        let _top = buffer_to_decimal(&data.drain(..2).collect());
        let _width = buffer_to_decimal(&data.drain(..2).collect());
        let _height = buffer_to_decimal(&data.drain(..2).collect());
        let compression_bit: Vec<u8> = data.drain(..1).collect();
        let compression_bit: String = decimal_to_binary(compression_bit[0]);
        let mut compression_bit: String = pad_start(compression_bit);
        // 是否有局部颜色列表
        // 0 – 没有 Local Color Table，如果 Global Color Table存在的话使用 Global Color Table
        // 1 – 有 Local Color Table，紧紧跟随在 Image Descriptor后面
        let _local_color_table_flag: String = compression_bit.drain(..1).collect();
        // _interlace
        // 0 – 图像没有隔行扫描
        // 1 – 图像隔行扫描
        let _interlace: String = compression_bit.drain(..1).collect();
        // _sort_flag
        // 0 – 没有排序
        // 1 – 按照重要次序递减排序
        let _sort_flag: String = compression_bit.drain(..1).collect();
        // 预留数据
        let _reserved: String = compression_bit.drain(..2).collect();
        // 局部颜色列表大小
        let _local_color_table_size: String = compression_bit.drain(..3).collect();
        let _local_color_table_size =
            2_usize.pow((binary_to_decimal(_local_color_table_size) + 1) as u32);
        if number_to_bool(string_to_decimal(&_local_color_table_flag)) {
            self.get_local_color_table(_local_color_table_size);
        }
        self.get_based_image_data();
    }
}
