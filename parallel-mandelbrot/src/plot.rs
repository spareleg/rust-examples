use rayon::prelude::*;
use std::ops::Deref;

pub trait Plotter<PX> {
    /// 0.0, 0.0 = upper left corner
    /// 1.0, 1.0 = lower right corner
    fn pixel_at(&self, horizontal: f64, vertical: f64) -> PX;
}

pub struct Plot<PX> {
    width: usize,
    height: usize,
    pixels: Vec<PX>,
}

impl<PX: Default + Copy> Plot<PX> {
    pub fn new(width: usize, height: usize) -> Self {
        Plot {
            width,
            height,
            pixels: vec![PX::default(); width * height],
        }
    }
}

impl<PX> Plot<PX> {
    pub fn render_parallel<PL>(&mut self, plotter: &PL)
    where
        PL: Plotter<PX> + Sync,
        PX: Send,
    {
        self.pixels
            .chunks_mut(self.width)
            .enumerate()
            .par_bridge()
            .for_each(|(row, pixels)| {
                let vertical = row as f64 / self.height as f64;
                for (col, px) in pixels.iter_mut().enumerate() {
                    let horizontal = col as f64 / self.width as f64;
                    *px = plotter.pixel_at(horizontal, vertical);
                }
            });
    }
}

impl<PX> Deref for Plot<PX> {
    type Target = [PX];
    fn deref(&self) -> &[PX] {
        &self.pixels
    }
}
