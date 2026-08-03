//! A modified example from the book: <https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/>

use clap::Parser;
use image::ColorType;
use mandelbrot::MandelbrotSet;
use plot::Plot;

use crate::parse::Pair;

mod mandelbrot;
mod parse;
mod plot;

/// Example: cargo run 1920x1080 -u=-1.2,0.35 -l=-1.1,0.20 out.png && open out.png
#[derive(Parser, Debug)]
struct Args {
    /// 1920x1080
    dimensions: String,
    /// -1.20,0.35
    #[arg(short, long)]
    upper_left: String,
    /// -1,0.20
    #[arg(short, long)]
    lower_right: String,
    /// out.png
    filename: String,
}

fn main() {
    let args = Args::parse();

    let Pair(width, height) = args
        .dimensions
        .parse()
        .expect("error parsing image dimensions");
    let upper_left: Pair<_> = args
        .upper_left
        .parse()
        .expect("error parsing upper left corner point");
    let lower_right: Pair<_> = args
        .lower_right
        .parse()
        .expect("error parsing lower right corner point");

    let mandelbrot = MandelbrotSet::new(upper_left.into(), lower_right.into());
    let mut plot = Plot::new(width, height);
    plot.render_parallel(&mandelbrot);

    image::save_buffer(
        &args.filename,
        plot.as_flattened(),
        width as u32,
        height as u32,
        ColorType::Rgb8,
    )
    .expect("error writing image file");
}
