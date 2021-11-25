extern crate image;

use image::GenericImageView;

fn main() {
    let mut img = image::open("images/profile-icon.png").unwrap();
    img.invert();
    img.save("invert.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
}
