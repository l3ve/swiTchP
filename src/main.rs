mod png;

fn main() {
    let mut images = png::Img::new("./src/images/white.png");
    let color_data = png::Img::get_color_data(&images);
    println!("{:?}", color_data);
    let _file = png::Img::create_png("./src/images/rust.png", &mut images);
}
