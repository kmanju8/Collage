mod color_averager;
mod downscaler;
use std::fs;
use std::collections::HashMap;

const SCALE: u32 = 20;

fn main() {
    // // DOWNSCALE MASTER
    // downscaler::downscale(
    //     "./src/myimage.JPG",
    //     format!("./output_images/testImproved{}.jpg", SCALE).as_str(),
    //     SCALE,
    // );

    // // DOWNSCALE SAMPLES
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
    let mut av_map: HashMap<i32, image::Rgb<u8>> = HashMap::new();
    for i in 0..50 {
        let av = color_averager::average(format!("./output_sample_images/{}.jpg", i).as_str());
        av_map.insert(i, av);
    }
    
    print!("{:?}", av_map)
}
