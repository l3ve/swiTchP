mod png;

fn main() {
    let mut imgae = png::Img::new("./src/images/js.png");
    // println!("{:?}", imgae);
    let _file = png::Img::write("./src/images/rust.png", &mut imgae);
}
