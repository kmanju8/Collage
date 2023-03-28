mod downscaler;

const SCALE: u32 = 20;

fn main() {
    downscaler::downscale("./src/myimage.JPG", SCALE)
}
