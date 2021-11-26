extern crate image;
extern crate rayon;
extern crate  job_scheduler;

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::env;
use std::process::Command;
use image::DynamicImage;
use image::GenericImageView;
use image::{Rgb, RgbImage};
use rand::Rng;
use rayon::prelude::*;
use std::fs;

const POST_URL: &str = "https://slack.com/api/users.setPhoto";
const IMAGE_SRC: &str = "images/profile-icon.png";
const OUTPUT_FILE: &str = "output.png";
const TOKEN_SRC: &str = "token.txt";
const CRON_INTERVAL: &str = "1/30 * * * * *";

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

fn build_absolute_path(src: &str) -> String {
    let current_dir = env::current_dir().unwrap();
    // for debug
    // println!("current dir: {}", current_dir.display());
    // println!("current exe: {}", env::current_exe().unwrap().display());
    let path = String::from(current_dir.to_str().unwrap()) + "/" + src;
    return path;
}

fn generate_image_and_post() {
    let src_path = build_absolute_path(IMAGE_SRC);
    let output_path = build_absolute_path(OUTPUT_FILE);
    let token_path = build_absolute_path(TOKEN_SRC);

    let img: DynamicImage = image::open(src_path).unwrap();

    let mut rng = rand::thread_rng();

    let r = rng.gen_range(0..255);
    let g = rng.gen_range(0..255);
    let b = rng.gen_range(0..255);

    let rgb = vec![r, g, b];

    let new_image = generate_image(img, rgb);

    new_image.save(&output_path).unwrap();

    // build token args
    let token = "token=".to_string();
    let raw_token = fs::read_to_string(token_path)
        .expect("error when read token txt");
    let token_arg = token + &raw_token;

    let image_arg = "image=@".to_string() + &output_path;

    let curl_command = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg(POST_URL)
        .arg("-F")
        .arg(token_arg)
        .arg("-F")
        .arg(image_arg)
        .arg("-F")
        .arg("pretty=1")
        .output()
        .expect("failed to execute.");

    let command_stdout = curl_command.stdout;
    let command_stderr = curl_command.stderr;

    println!("{}", std::str::from_utf8(&command_stdout).unwrap());
    println!();
    println!("{}", std::str::from_utf8(&command_stderr).unwrap());
    println!();
}

fn exec_scheduler() {
    let mut scheduler = JobScheduler::new();
    scheduler.add(Job::new(CRON_INTERVAL.parse().unwrap(), || {
        generate_image_and_post();
    }));
    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    exec_scheduler();
}
