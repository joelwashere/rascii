//A rust app that turns images into ascii art
//Author: Joel Williams

use std;
use image;

struct Config {
    file_path: String,
    width: Option<u32>,
    height: Option<u32>
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = args.next().ok_or("missing file_path")?;
        let mut width: Option<u32> = None;
        let mut height: Option<u32> = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--width" => {
                    let value = args.next().ok_or("--width requires a value")?;
                    width = Some(value.parse().map_err(|_| "--width expects a positive integer")?);
                }
                "--height" => {
                    let value = args.next().ok_or("--height requires a value")?;
                    height = Some(value.parse().map_err(|_| "--height expects a positive integer")?);
                }
                _ => return Err("unrecognized argument")
            }
        }

        Ok(Config { file_path, width, height })
    
    }
}

fn img_to_ascii(config: &Config) -> Result<String, image::ImageError> {

    const ASCII_VALUES: [char; 10] = ['@','#','S','%','?','*','+',';','.',' '];
    
    let img = image::ImageReader::open(&config.file_path)?.decode()?;
    
    let grayscale_img: image::GrayImage = img.grayscale().into_luma8();
    let (w, h) = grayscale_img.dimensions();
    
    let dimensions: (u32, u32) = (config.width.unwrap_or(100), config.height.unwrap_or(100));

    let mut ascii_img = String::with_capacity((w * h + h) as usize);

    for y in (0..h).step_by((h/(dimensions.1 / 2)) as usize) {
        for x in (0..w).step_by((w/dimensions.0) as usize) {
            let lum = grayscale_img.get_pixel(x, y).0[0] as usize;
            let index = lum * (ASCII_VALUES.len() - 1) / 255;

            ascii_img.push(ASCII_VALUES[index]);
        }
        ascii_img.push('\n');
    }

    Ok(ascii_img)
}

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(|e| {
        eprintln!("Issue parsing arguments: {e}");
        std::process::exit(1);
    });
    
    println!("Converting {} into an ASCII image!", config.file_path);

    match img_to_ascii(&config) {
        Ok(img) => println!("{}", img),
        Err(err) => eprintln!("Failed to convert image: {}", err)
    }
}
