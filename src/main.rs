mod png;

fn main() {
    let imgae = png::Img::new("./src/images/white.png");
    println!("{:?}", imgae);
}
