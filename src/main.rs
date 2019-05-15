
use image::{DynamicImage, GenericImage, GenericImageView, Rgb, Pixel};

fn main() {
    let mut img = DynamicImage::new_rgb8(800, 600);

    let w = img.width();
    let h = img.height();
    for y in 0..h {
        for x in 0..w {
            let r = (255 * x / w) as u8;
            let g = (255 * y / h) as u8;
            let b = 100;
            let color = Rgb([r, g, b]);
            img.put_pixel(x, y, color.to_rgba());
        }
    }

    img.save("output.png").unwrap();
}
