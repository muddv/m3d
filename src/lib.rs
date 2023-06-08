use std::fs::File;
use std::io::{ErrorKind, Write};
use u32;

pub struct Canvas {
    pub height: usize,
    pub width: usize,
}

pub struct Circle {
   pub center_x: usize,
   pub center_y: usize,
   pub radius: usize,
}

pub struct Rectangle {
   pub x0: usize,
   pub y0: usize,
   pub width: usize,
   pub height: usize,
}

pub fn fill(color: u32, canvas: &Canvas) -> Vec<u32> {
    let mut pixels = vec![0; canvas.width * canvas.height];
    for i in 0..canvas.width * canvas.height {
        pixels[i] = color;
    }
    pixels
}

pub fn fill_rectangle(
    mut pixels: Vec<u32>,
    color: u32,
    canvas: &Canvas,
    rectangle: Rectangle,
) -> Vec<u32> {
    for dy in 0..canvas.height {
        let y = rectangle.y0 + dy;
        if 0 < y && y < rectangle.height {
            for dx in 0..canvas.width {
                let x = rectangle.x0 + dx;
                if 0 < x && x < rectangle.width {
                    pixels.insert((y * rectangle.width).try_into().unwrap(), color);
                }
            }
        }
    }
    pixels
}

pub fn fill_circle(mut pixels: Vec<u32>, color: u32, canvas: &Canvas, circle: Circle) -> Vec<u32> {
    let x1: i32 = circle.center_x as i32 - circle.radius as i32;
    let y1: i32 = circle.center_y as i32 - circle.radius as i32;
    let x2: i32 = circle.center_x as i32 + circle.radius as i32;
    let y2: i32 = circle.center_y as i32 + circle.radius as i32;

    for y in y1..y2 {
        if 0 < y && y < canvas.height as i32 {
            for x in x1..x2 {
                if 0 < x && x < canvas.width as i32 {
                    let dx = x - circle.center_x as i32;
                    let dy = y - circle.center_y as i32;
                    // this produces square
                    //if f64::sqrt((dx*dx + dy*dy) as f64) <= circle.radius as f64 *circle.radius as f64 {
                    if (dx*dx + dy*dy) <= circle.radius as i32 *circle.radius as i32 {
                        pixels.insert(y as usize * canvas.width + x as usize, color);
                    }
                }
            }
        }
    }
    pixels
}

pub fn save_to_ppm(pixels: Vec<u32>, filepath: String, canvas: &Canvas) {
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

    match write!(image_file, "P6\n{} {} 255\n", canvas.width, canvas.height) {
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
