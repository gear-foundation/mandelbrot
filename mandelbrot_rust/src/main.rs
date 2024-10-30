use image::{ImageBuffer, Rgb};
use std::time::Instant;

fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = c_re;
    let mut z_im = c_im;
    for i in 0..max_iter {
        if z_re * z_re + z_im * z_im > 4.0 {
            return i;
        }
        let new_re = z_re * z_re - z_im * z_im + c_re;
        z_im = 2.0 * z_re * z_im + c_im;
        z_re = new_re;
    }
    max_iter
}

fn generate_mandelbrot(width: u32, height: u32, max_iter: u32, zoom: f64, center_re: f64, center_im: f64) -> (ImageBuffer<Rgb<u8>, Vec<u8>>, u32) {
    let mut img = ImageBuffer::new(width, height);
    let mut points_found = 0;

    let scale_x = 4.0 / (width as f64) / zoom;
    let scale_y = 4.0 / (height as f64) / zoom;

    for x in 0..width {
        for y in 0..height {
            
            let c_re = center_re + (x as f64 * scale_x - 2.0 / zoom);
            let c_im = center_im + (y as f64 * scale_y - 2.0 / zoom);

            let iter = mandelbrot(c_re, c_im, max_iter);
            if iter == max_iter {
                points_found += 1;
            }

            let pixel = img.get_pixel_mut(x, y);
            let color = if iter == max_iter {
                [0, 0, 0]
            } else {
                let ratio = iter as f64 / max_iter as f64;
                [
                    (255.0 * ratio) as u8,
                    (255.0 * (1.0 - ratio)) as u8,
                    (255.0 * (0.5 - ratio / 2.0)) as u8,
                ]
            };
            *pixel = Rgb(color);
        }
    }
    (img, points_found)

fn main() {
    let width = 2000;
    let height = 2000;
    let max_iter = 1000;
    let zoom = 1.0;
    let center_re = 0.0;
    let center_im = 0.0;

    let start_time = Instant::now();

    let (img, points_found) = generate_mandelbrot(width, height, max_iter, zoom, center_re, center_im);

    let duration = start_time.elapsed();
    println!("Points found in the Mandelbrot set: {}", points_found);
    println!("Time taken: {:?}", duration);

    img.save("mandelbrot_zoom.png").unwrap();
}