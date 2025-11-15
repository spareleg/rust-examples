//! A modified example from the book: <https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/>

use std::env;

use mandelbrot::MandelbrotSet;
use plot::Plot;

mod mandelbrot;
mod parse;
mod plot;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER-LEFT LOWER-RIGHT", args[0]);
        eprintln!("Example: {} out.png 1920x1080 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let (width, height) = parse::pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse::complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse::complex(&args[4]).expect("error parsing lower right corner point");

    let mandelbrot = MandelbrotSet::new(upper_left, lower_right);
    let mut plot = Plot::new(width, height);
    plot.render_parallel(&mandelbrot)
        .expect("error rendering plot in parallel");
    plot.save(&args[1]).expect("error writing image file");
}
