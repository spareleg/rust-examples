use num::Complex;

use crate::plot::Plotter;

pub struct MandelbrotSet {
    upper_left: Complex<f64>,
    width: f64,
    height: f64,
}

impl MandelbrotSet {
    pub fn new(upper_left: Complex<f64>, lower_right: Complex<f64>) -> Self {
        MandelbrotSet {
            upper_left,
            width: lower_right.re - upper_left.re,
            height: upper_left.im - lower_right.im,
        }
    }
}

impl Plotter<u8> for MandelbrotSet {
    fn pixel_at(&self, horizontal: f64, vertical: f64) -> u8 {
        let point = Complex {
            re: self.upper_left.re + horizontal * self.width,
            im: self.upper_left.im - vertical * self.height,
        };
        escape_time(point, 255).map_or(0, |count| 255 - count as u8)
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member), return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}
