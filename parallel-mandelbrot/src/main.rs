//! A modified example from the book: <https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/>

use clap::Parser;
use image::ColorType;
use mandelbrot::MandelbrotSet;
use plot::Plot;

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

    let (width, height) =
        parse::pair(&args.dimensions, 'x').expect("error parsing image dimensions");
    let upper_left =
        parse::complex(&args.upper_left).expect("error parsing upper left corner point");
    let lower_right =
        parse::complex(&args.lower_right).expect("error parsing lower right corner point");

    let mandelbrot = MandelbrotSet::new(upper_left, lower_right);
    let mut plot = Plot::new(width, height);
    plot.render_parallel(&mandelbrot)
        .expect("error rendering plot in parallel");

    image::save_buffer(
        &args.filename,
        &plot,
        width as u32,
        height as u32,
        ColorType::L8,
    )
    .expect("error writing image file");
}
