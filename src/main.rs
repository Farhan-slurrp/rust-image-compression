use image_compression::compress;

mod image_compression;

fn main() {
    compress(image_compression::Quality::Medium);
}
