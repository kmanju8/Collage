mod downscaler;

const SCALE: u32 = 20;

fn main() {
    downscaler::downscale(
        "./src/myimage.JPG",
        format!("./output_images/testImproved{}.jpg", SCALE).as_str(),
        SCALE,
    )
}
