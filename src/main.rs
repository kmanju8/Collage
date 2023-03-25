use image::{ImageBuffer, Rgb};
use std::env;

const SCALE: u32 = 15;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

fn print_dimensions(img: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    println!(
        "\nPixel {}: {:?} width: {} height: {}",
        0,
        &img[(0, 0)],
        img.dimensions().0,
        img.dimensions().1
    );
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    print!("Hello, world!, {}", get_current_working_dir());
    let img = image::open("./src/myimage.JPG").unwrap();

    img.save("./output_images/test.jpg").unwrap();

    let rgb_img = img.to_rgb8();

    print_dimensions(rgb_img.clone());

    let (width, height) = rgb_img.dimensions();

    let mut sum_of_squares =
        vec![vec![[0_u32; 3]; ((height / SCALE) + 1) as usize]; ((width / SCALE) + 1) as usize];
    let mut simple_img =
        ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width / SCALE + 1, height / SCALE + 1);

    print!(
        "sum of squares size is {}x{}",
        sum_of_squares.len(),
        sum_of_squares[0].len()
    );
    print_dimensions(simple_img.clone());

    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        sum_of_squares[(x / SCALE) as usize][(y / SCALE) as usize][0] += (pixel[0] as u32).pow(2);
        sum_of_squares[(x / SCALE) as usize][(y / SCALE) as usize][1] += (pixel[1] as u32).pow(2);
        sum_of_squares[(x / SCALE) as usize][(y / SCALE) as usize][2] += (pixel[2] as u32).pow(2);
    }

    for (x, row) in sum_of_squares.iter_mut().enumerate() {
        for (y, pixel) in row.iter_mut().enumerate() {
            // print!("x is {} y is {}", x, y);
            let new_pixel = Rgb(pixel.map(|x| ((x as f32).sqrt() as u32 / (SCALE)) as u8));
            simple_img.put_pixel(x as u32, y as u32, new_pixel);
        }
    }

    simple_img
        .save(format!("./output_images/testImproved{}.jpg", SCALE))
        .unwrap();
}
