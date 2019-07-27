use palette::{Hsv, rgb};

pub struct Config<'a> {
    pub width: u32,
    pub height: u32,
    pub max_iterations: u32,
    pub buffer: &'a mut Vec<u8>,
}

impl<'a> Config<'a> {
    pub fn new(width: u32, height: u32, max_iterations: u32, buffer: &'a mut Vec<u8>) -> Config {
        buffer.reserve((width * height * 3u32) as usize);
        Config {
            width,
            height,
            max_iterations,
            buffer,
        }
    }
}

pub fn run(config: Config) {
    let Config{ width, height, max_iterations, buffer } = config;
    for row in 0..height {
        for col in 0..width {
            let real_c = (col as f64 - width as f64 / 2.0) * 4.0 / width as f64;
            let imaginary_c = (row as f64 - height as f64 / 2.0) * 4.0 / width as f64;

            let mut x: f64 = 0.0;
            let mut y: f64 = 0.0;
            let mut iterations = 0;
            while x.powi(2) + y.powi(2) < 4.0 && iterations < max_iterations {
                let x_new = x.powi(2) - y.powi(2) + real_c;
                y = 2.0 * x * y + imaginary_c;
                x = x_new;
                iterations += 1;
            }
            if iterations < max_iterations {
                let zn = (x.powi(2) + y.powi(2)).sqrt();
                let nsmooth = iterations as f64 + 1.0f64 - zn.abs().log10().log10() / 2.0f64.log10();
                let color: rgb::Rgb = rgb::Rgb::from(Hsv::new(0.95 + 10.0 * nsmooth as f32, 0.6, 1.0));
                let (r, g, b) = color.into_components();
                buffer.push((r * 255.0) as u8);
                buffer.push((g * 255.0) as u8);
                buffer.push((b * 255.0) as u8);

            } else {
                for _ in 0..3 {
                    buffer.push(0);
                }
            }
        }
    }
}