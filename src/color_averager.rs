use image::{Pixel, Rgb};

// take an input image, then return average rgb
pub fn average(path: &str) -> Rgb<u8> {
    let img = image::open(path).unwrap();

    let rgb_img = img.to_rgb8();

    let mut total = [0_f64; 3];

    for (_, _, pixel) in rgb_img.enumerate_pixels() {
        for (i, color) in pixel.channels().iter().enumerate() {
            total[i] += (*color as f64).powi(2);
        }
    }

    let (height, width) = rgb_img.dimensions();
    let pixel_count = (height as f64) * (width as f64);

    Rgb(total.map(|x| (x / pixel_count).sqrt() as u8))
}
