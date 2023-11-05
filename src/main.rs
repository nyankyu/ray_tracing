use image::{ImageBuffer, ImageError, Rgb};
use std::{fs::create_dir_all, path::Path};

const SAVE_DIR: &str = "images";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    make_dir()?;

    let mut imgbuf = ImageBuffer::new(1024, 1024);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([((x + y) % 197) as u8, ((x * y) % 199) as u8, 0]);
    }

    save(imgbuf)?;

    Ok(())
}

fn make_dir() -> Result<(), std::io::Error> {
    let save_dir = Path::new(SAVE_DIR);
    create_dir_all(save_dir)?;

    Ok(())
}

fn save(imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Result<(), ImageError> {
    let path = Path::new(SAVE_DIR).join("image.png");
    imgbuf.save(path)?;

    Ok(())
}
