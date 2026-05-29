use image::{Rgba, RgbaImage};

use super::palette::Color;

pub fn fill_background(image: &mut RgbaImage, color: Color) {
    for pixel in image.pixels_mut() {
        *pixel = Rgba(color);
    }
}

pub fn fill_rect(image: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let (w, h) = (image.width() as i32, image.height() as i32);
    let x0 = x0.clamp(0, w);
    let x1 = x1.clamp(0, w);
    let y0 = y0.clamp(0, h);
    let y1 = y1.clamp(0, h);
    for y in y0..y1 {
        for x in x0..x1 {
            image.put_pixel(x as u32, y as u32, Rgba(color));
        }
    }
}

pub fn stroke_rect(image: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    if x1 <= x0 || y1 <= y0 {
        return;
    }
    fill_rect(image, x0, y0, x1, y0 + 1, color);
    fill_rect(image, x0, y1 - 1, x1, y1, color);
    fill_rect(image, x0, y0, x0 + 1, y1, color);
    fill_rect(image, x1 - 1, y0, x1, y1, color);
}

pub fn draw_grid(image: &mut RgbaImage, tile_size: u32, color: Color) {
    let (w, h) = (image.width(), image.height());
    if tile_size == 0 {
        return;
    }
    let mut x = 0;
    while x < w {
        for y in 0..h {
            image.put_pixel(x, y, Rgba(color));
        }
        x += tile_size;
    }
    let mut y = 0;
    while y < h {
        for x in 0..w {
            image.put_pixel(x, y, Rgba(color));
        }
        y += tile_size;
    }
}
