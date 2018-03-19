mod png;

fn main() {
    let mut images = png::Img::new("./src/images/white.png");
    let _file = png::Img::create_png("./src/images/rust.png", &mut images);
}
