extern crate image;
extern crate rayon;

use image::DynamicImage;
use image::GenericImageView;
use image::{Rgb, RgbImage};
use rand::Rng;
use rayon::prelude::*;

fn generate_image(src_img: DynamicImage, rgb: Vec<u8>) -> RgbImage {
    let (width, height) = src_img.dimensions();

    let mut img = RgbImage::new(width, height);

    let min = rgb.iter().fold(0, |m, v| *v.min(&m));
    let max = rgb.iter().fold(0, |m, v| *v.max(&m));
    let complementary_base_color = max + min;

    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let current_profile_color = src_img.get_pixel(*x, *y);
            if current_profile_color[0] > 127 {
                pixel[0] = rgb[0];
                pixel[1] = rgb[1];
                pixel[2] = rgb[2];
            } else {
                pixel[0] = complementary_base_color - rgb[0];
                pixel[1] = complementary_base_color - rgb[1];
                pixel[2] = complementary_base_color - rgb[2];
            }
        });

    return img;
}

fn main() {
    let src_path = "images/profile-icon.png";
    let output_path = "output.png";

    let img: DynamicImage = image::open(src_path).unwrap();

    let mut rng = rand::thread_rng();

    let r = rng.gen_range(0..255);
    let g = rng.gen_range(0..255);
    let b = rng.gen_range(0..255);

    let rgb = vec![r, g, b];

    let new_image = generate_image(img, rgb);

    new_image.save(output_path).unwrap();
}
