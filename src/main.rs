use std::env;

use image::{self, GrayImage};

fn img_to_ascii(file_path: &str) -> Result<String, image::ImageError> {

    const ASCII_VALUES: [char; 10] = ['@','#','S','%','?','*','+',';','.',' '];
    let dimensions: (u32, u32) = (100, 100);
    
    let img = image::ImageReader::open(file_path)?
        .decode().expect("Couldn't open file: {:file_path?}");

    let grayscale_img: GrayImage = img.grayscale().into_luma8();
    let (w, h) = grayscale_img.dimensions();
    let mut ascii_img = String::with_capacity((w * h + h) as usize);

    for y in (0..h).step_by((h/(dimensions.1 / 2)) as usize) {
        for x in (0..w).step_by((w/dimensions.0) as usize) {
            let lum = grayscale_img.get_pixel(x, y).0[0] as usize;
            let index = lum * (ASCII_VALUES.len() - 1) / 255;

            ascii_img.push(ASCII_VALUES[index]);
        }
        ascii_img.push('\n');
    }

    return Ok(ascii_img)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    //Config
    let file_path = &args[1];
    //let width = &args[2];
    //let height = &args[3];
    
    println!("Converting {} into an ASCII image!", file_path);

    let img = img_to_ascii(file_path).unwrap();
    
    println!("{}", img)
}
