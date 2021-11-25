extern crate image;

use image::DynamicImage;
use image::GenericImageView;
use image::Rgba;

fn main() {
    // let mut img = image::open("images/profile-icon.png").unwrap();
    // img.invert();
    // img.save("invert.png").unwrap();
    // println!("dimensions {:?}", img.dimensions());

    let src_path = "images/profile-icon.png";

    let img: DynamicImage = image::open(src_path).unwrap();

    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel: Rgba<u8> = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            // for debug
            // println!("{} {} {}", r, g, b);
        }
    }
}
