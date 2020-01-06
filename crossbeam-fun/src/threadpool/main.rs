extern crate threadpool;
extern crate num;
extern crate num_cpus;
extern crate image;

use std::sync::mpsc::{channel};
use threadpool::ThreadPool;
use image::{ImageBuffer};

fn main() -> Result<(),()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || for x in 0..width {
                         let i = 1;
                         let pixel = image::Rgb([i, 1, 1]);
                         tx.send((x, y, pixel)).expect("Could not send data!");
                     });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv().unwrap();
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png");
    Ok(())
}