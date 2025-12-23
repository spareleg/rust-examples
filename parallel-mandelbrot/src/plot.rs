use std::{io, ops::Deref, path::Path, thread};

use image::{ImageBuffer, ImageResult, Luma};

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
    pub fn render_parallel<PL>(&mut self, plotter: &PL) -> io::Result<()>
    where
        PL: Plotter<PX> + Sync,
        PX: Send,
    {
        let (width, height) = (self.width, self.height);
        let threads_n = thread::available_parallelism()?.get();
        let height_per_thread = height.div_ceil(threads_n);
        let thread_pixels = self.pixels.chunks_mut(height_per_thread * width);
        thread::scope(|scope| {
            for (i, pixels) in thread_pixels.enumerate() {
                let band_top = height_per_thread * i;
                let band_height = pixels.len() / width;
                scope.spawn(move || {
                    println!("Thread #{i} put to work");
                    for band_row in 0..band_height {
                        let vertical = (band_top + band_row) as f64 / height as f64;
                        for col in 0..width {
                            let horizontal = col as f64 / width as f64;
                            pixels[band_row * width + col] = plotter.pixel_at(horizontal, vertical);
                        }
                    }
                    println!("Thread #{i} finished");
                });
            }
        });
        Ok(())
    }
}

impl<PX> Deref for Plot<PX> {
    type Target = [PX];
    fn deref(&self) -> &[PX] {
        &self.pixels
    }
}

impl Plot<u8> {
    pub fn save(&self, path: impl AsRef<Path>) -> ImageResult<()> {
        let buf: ImageBuffer<Luma<_>, _> =
            ImageBuffer::from_raw(self.width as u32, self.height as u32, &self[..])
                .expect("No way buffer sizes don't match");
        buf.save(path)
    }
}
