extern crate image;
extern crate rayon;
extern crate job_scheduler;

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
use chrono::Local;

const POST_URL: &str = "https://slack.com/api/users.setPhoto";
const IMAGE_SRC_DEFAULT: &str = "images/profile-icon-default.png";
const IMAGE_SRC_OPEN_EYES: &str = "images/profile-icon-open-eyes.png";
const OUTPUT_FILE: &str = "output.png";
const TOKEN_SRC: &str = "token.txt";
// const CRON_INTERVAL: &str = "1/60 * * * * *";
// const CRON_INTERVAL: &str = "0 10/5 * * * *";
const CRON_INTERVAL: &str = "0 1/5 * * * * *";

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
    let src_path_default = build_absolute_path(IMAGE_SRC_DEFAULT);
    let src_path_open_eyes = build_absolute_path(IMAGE_SRC_OPEN_EYES);
    let output_path = build_absolute_path(OUTPUT_FILE);
    let token_path = build_absolute_path(TOKEN_SRC);

    let mut rng = rand::thread_rng();

    let img_src_path;
    let img_src_dice = rng.gen_range(0.0..1.0);
    if img_src_dice > 0.99  {
	img_src_path = src_path_open_eyes;
    } else {
 	img_src_path = src_path_default;
    }

    let img: DynamicImage = image::open(img_src_path).unwrap();

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

    let now = Local::now();

    println!("formatted time: {}", now.format("%Y-%m-%d %H:%M:%S"));
    println!("img src dice: {}", img_src_dice);
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
    // exec once
    generate_image_and_post();
    exec_scheduler();
}
