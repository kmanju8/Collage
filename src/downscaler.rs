use image::{ImageBuffer, Pixel, Rgb};

// fn print_dimensions(img: ImageBuffer<Rgb<u8>, Vec<u8>>) {
//     println!(
//         "\nPixel {}: {:?} width: {} height: {}",
//         0,
//         &img[(0, 0)],
//         img.dimensions().0,
//         img.dimensions().1
//     );
// }

pub fn downscale(master_path: &str, scale: u32){
    let img = image::open(master_path).unwrap();

    img.save("./output_images/test.jpg").unwrap();

    let rgb_img = img.to_rgb8();


    let (width, height) = rgb_img.dimensions();

    let mut sum_of_squares =
        vec![vec![[0_u32; 3]; ((height / scale) + 1) as usize]; ((width / scale) + 1) as usize];
    let mut simple_img =
        ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width / scale + 1, height / scale + 1);

    print!(
        "sum of squares size is {}x{}",
        sum_of_squares.len(),
        sum_of_squares[0].len()
    );

    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        for (total_sqr, color) in sum_of_squares[(x / scale) as usize][(y / scale) as usize]
            .iter_mut()
            .zip(pixel.channels().iter())
        {
            *total_sqr += (*color as u32).pow(2);
        }
    }

    for (x, row) in sum_of_squares.iter_mut().enumerate() {
        for (y, pixel) in row.iter_mut().enumerate() {
            let new_pixel = Rgb(pixel.map(|x| ((x as f32).sqrt() as u32 / (scale)) as u8));
            simple_img.put_pixel(x as u32, y as u32, new_pixel);
        }
    }

    simple_img
        .save(format!("./output_images/testImproved{}.jpg", scale))
        .unwrap();
}