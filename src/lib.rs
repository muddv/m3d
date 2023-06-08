use std::fs::File;
use std::io::{ErrorKind, Write};
use u32;

//struct Canvas {
//    height: usize,
//    width: usize,
//}

//const WIDTH: usize = 800;
//const HEIGHT: usize = 600;
//const CANVAS: Canvas = Canvas {
//    width: WIDTH,
//    height: HEIGHT,
//};


// signature: pixels?, color, width, height
pub fn fill(color: u32, width: usize, height: usize) -> Vec<u32> {
    let mut pixels = vec![0; width * height];
    for i in 0..width * height {
        pixels[i] = color;
    }
    pixels
}

// signature: pixels, color, width, height, x0, y0, pw, ph
pub fn fill_rectangle(
    mut pixels: Vec<u32>,
    color: u32,
    width: u32,
    height: u32,
    x0: u32,
    y0: u32,
    rectangle_width: u32,
    rectengle_height: u32,
) -> Vec<u32> {
    for dy in 0..height {
        let y = y0 + dy;
        if 0 < y && y < rectengle_height {
            for dx in 0..width {
                let x = x0 + dx;
                if 0 < x && x < rectangle_width {
                    pixels.insert((y * rectangle_width).try_into().unwrap(), color);
                }
            }
        }
    }
    pixels
}

pub fn save_to_ppm(pixels: Vec<u32>, filepath: String, width: usize, height: usize) {
    let mut image_file = match File::create(&filepath) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&filepath) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    match write!(image_file, "P6\n{} {} 255\n", width, height) {
        Ok(w) => w,
        Err(e) => panic!("Problem writing to file: {:?}", e),
    };

    for i in pixels {
        let b = u32::to_be_bytes(i);
        for j in 0..3 {
            match image_file.write(&[b[j]]) {
                Ok(w) => w,
                Err(e) => panic!("Problem writing to file: {:?}", e),
            };
        }
    }
}
