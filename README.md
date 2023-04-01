# Collage

Intended to be a non-trivial slightly artsy project as a gift. And like probably half the projects out there these days, to learn Rust. Program will take in a collection of images as input + one master image, then recreate the master image out of the provided collection of images (kind of like ASCII art).

Basic terminology:
Master image : the image we wish to recreate as a collage of photos
Sample collection: images used to recreate master

## System Overview

Master image -> downscale
Sample images -> find average RGB value for each image (different averages to be tried) -> Output: Hashmap where key=id_of_sample and value = average RGB

Map each downscaled master pixel to most similar sample average color. Output will be image pixel array with rgb values replaced by keys from hashmap of previous step.

Stitch together images to make one mega-sized collage.

### Progress

- [x] Master downscale
  - [ ] Tweak x/y scaling
- [x] Sample downscale
- [x] Sample averager
  - [x] Convert output to hashmap
- [ ] Pixel-to-pic mapper
- [ ] Collage stitcher

## Downscaler

So I started out with some basic blurring by just averaging out rgb values.
This was pretty ineffective, giving essentially a 'deep-fried' effect.

This video https://www.youtube.com/watch?v=LKnqECcg6Gw&t=219s&ab_channel=minutephysics pretty much explains a lot of the problem.

Will illustrate the difference between accounting for this difference with a before/after.

### For purposes of collage

currently average squares of pixels: but most of my photos are 4:3 aspect ratio! So need to have a separate scale factor for x and y axis, such that resulting downscaled image is (approx) a square. This was, when each square is replaced by a 4:3 picture, the final collage is the same aspect ratio as the master image.

### High Dynamic Range (HDR)

I could add a HDR step prior/after downscaling the master image. HDR essentially looks at the distribution of colors, and if there is an overabundance up of lighter/darker colors (and lack of the other) it ups the contrast of the colours e.g. and example looking at just red values:


| Pixel | 1 | 2 | 3 |
| --- |--- | ---| --- |
| Old R | 0 | 1 | 2 |
| HDR R | 0 | 128 | 256 |

(Note: may have to scale radially than by each cartisian color coordinate. Worth experimenting.)

This is definitely not a step that is required - but it'll lead to a clearer final collage which would certainly be much needed. Also might just use Google Photos inbuilt HDR function lol.

## RGB Average Identifier

For efficient processing, thinking of having a preproc stage where every sample image is downscaled first: this needs to be done anyway for the final collage so may as well get it done here to make RGB averages easier to calculate.

Need to make same consideration as downscaler. Average has to be root sum of squares mean, not linear mean. This will play in later stages as well.

## Image Mapper

Distance will be done by pretty much Pythagoras. Again, probably going to have to calculate this in terms of squares, so instead of distance being a function of dR<sup>2</sup> +dG<sup>2</sup> +dB<sup>2</sup>, it'll more likely be dR<sup>4</sup> +dG<sup>4</sup> +dB<sup>4</sup>.

## Pixel-to-Image Mapper

TODO

## Stitching

200x125 image (very legible) comes to a mere 10KB.
That's a total of 25000 pixels.
25000 x 10kKB = 250000KB = 250MB image. Pretty hefty for an image, but shouldn't crash a modern computer.

sample(200x125) x master(125x125) = 25000x15625 is rough expected size.
max JPEG size is 65535<sup>2</sup> so this fits, but a png could be bigger. Can play around with this.
