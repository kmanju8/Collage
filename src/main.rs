mod color_averager;
mod downscaler;
use std::fs;

const SCALE: u32 = 20;

fn main() {
    // DOWNSCALE MASTER
    // downscaler::downscale(
    //     "./src/myimage.JPG",
    //     format!("./output_images/testImproved{}.jpg", SCALE).as_str(),
    //     SCALE,
    // );

    // DOWNSCALE SAMPLES
    // let paths = fs::read_dir("./sample_images").unwrap();
    // for (i, path) in paths.enumerate() {
    //     downscaler::downscale(
    //         path.unwrap().path().display().to_string().as_str(),
    //         format!("./output_sample_images/{}.jpg", i).as_str(),
    //         SCALE,
    //     );
    //     println!("{} images downscaled", i+1);
    // }

    // CALCULATE AVERAGE COLOUR OF SAMPLES
    // for i in 0..50 {
    //     let av = color_averager::average(format!("./output_sample_images/{}.jpg", i).as_str());
    //     print!("{:?}", av)
    // }

}
