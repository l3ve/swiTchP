use std::io::Result;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let image_buffer = get_image_buffer().unwrap();
    println!("{:?}", image_buffer);

}
fn get_image_buffer() -> Result<Vec<u8>> {
    let path = Path::new("./src/images/js.png");
    let mut f = File::open(&path)?;
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}
