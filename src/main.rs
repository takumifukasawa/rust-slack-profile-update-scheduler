extern crate image;
extern crate rayon;

use image::DynamicImage;
use image::GenericImageView;
use image::{ImageBuffer, Rgb, RgbImage, Rgba};
use rand::Rng;
use rayon::prelude::*;

fn generate_image(src_img: DynamicImage, rgb: Vec<u8>) -> RgbImage {
    let (width, height) = src_img.dimensions();

    let mut img = RgbImage::new(width, height);

    let min = rgb.iter().fold(0, |m, v| *v.min(&m));
    let max = rgb.iter().fold(0, |m, v| *v.max(&m));
    let base_color = max + min;

    // let pixel: Rgba<u8> = img.get_pixel(x, y);
    // let r = pixel[0];
    // let g = pixel[1];
    // let b = pixel[2];
    // let rgb = vec![r, g, b];
    // let min = rgb.iter().fold(0, |m, v| *v.min(&m));
    // let max = rgb.iter().fold(0, |m, v| *v.max(&m));

    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
            // pixel[0] = base_color - r;
            // pixel[1] = base_color - g;
            // pixel[2] = base_color - b;
        });

    return img;
}

fn main() {
    // let mut img = image::open("images/profile-icon.png").unwrap();
    // img.invert();
    // img.save("invert.png").unwrap();
    // println!("dimensions {:?}", img.dimensions());

    let src_path = "images/profile-icon.png";

    let img: DynamicImage = image::open(src_path).unwrap();

    // let (width, height) = img.dimensions();

    // let mut new_image: image::RgbImage = image::ImageBuffer::new(width, height);

    let mut rng = rand::thread_rng();

    let r = rng.gen_range(0..255);
    let g = rng.gen_range(0..255);
    let b = rng.gen_range(0..255);

    let rgb = vec![r, g, b];

    let new_image = generate_image(img, rgb);

    new_image.save("output.png").unwrap();

    // for y in 0..height {
    //     for x in 0..width {
    //         let pixel: Rgba<u8> = img.get_pixel(x, y);
    //         let r = pixel[0];
    //         let g = pixel[1];
    //         let b = pixel[2];
    //         let rgb = vec![r, g, b];
    //         let min = rgb.iter().fold(0, |m, v| *v.min(&m));
    //         let max = rgb.iter().fold(0, |m, v| *v.max(&m));

    //         let new_r =
    //         // let max = &rgbIterator.fold(0, |m, &v| v.max(m));

    //         // for debug
    //         // println!("{:?}, {:?}", min, max);

    //         // let base =
    //         // for debug
    //         // println!("{} {} {}", r, g, b);

    //         break;
    //     }
    //         break;
    // }
}
