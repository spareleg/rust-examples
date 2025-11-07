//! A slightly modified example from the book:
//! <https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/>

use std::{env, thread};

mod mandelbrot;
mod parse;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER-LEFT LOWER-RIGHT", args[0]);
        eprintln!(
            "Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let (width, height): (usize, usize) =
        parse::pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse::complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse::complex(&args[4]).expect("error parsing lower right corner point");

    let threads_n = thread::available_parallelism()
        .expect("error querying the amount of parallelism available")
        .get();
    let height_per_thread = height.div_ceil(threads_n);

    let mut pixels = vec![0; width * height];
    let pixels_bands = pixels.chunks_mut(height_per_thread * width);
    thread::scope(|spawner| {
        for (i, pixels) in pixels_bands.enumerate() {
            let top = height_per_thread * i;
            let band_height = pixels.len() / width;
            let band_upper_left =
                mandelbrot::pixel_to_point((width, height), (0, top), upper_left, lower_right);
            let band_lower_right = mandelbrot::pixel_to_point(
                (width, height),
                (width, top + band_height),
                upper_left,
                lower_right,
            );
            spawner.spawn(move || {
                mandelbrot::render(
                    pixels,
                    width,
                    band_height,
                    band_upper_left,
                    band_lower_right,
                );
            });
        }
    });

    image::save_buffer(
        &args[1],
        &pixels,
        width as u32,
        height as u32,
        image::ColorType::L8,
    )
    .expect("error writing PNG file");
}
