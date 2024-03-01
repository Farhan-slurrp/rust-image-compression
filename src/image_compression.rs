use image::{open, DynamicImage, GrayImage, Luma};
use nalgebra::{DMatrix, Dyn, Matrix, VecStorage};
use std::path::Path;

#[allow(dead_code)]
pub enum Quality {
    Low,
    Medium,
    High,
}

pub fn compress(quality: Quality) {
    let path = Path::new("./test/cat.jpeg");
    let image_open = open(path).unwrap();
    let img = image_open.clone().into_luma8();
    let (width, height) = img.dimensions();
    let mut img_mtx = DMatrix::<f64>::zeros(height as usize, width as usize);
    for y in 0..height {
        for x in 0..width {
            let rgb_value = img.get_pixel(x, y)[0] as f64;
            img_mtx[(y as usize, x as usize)] = rgb_value;
        }
    }

    let svd = img_mtx.svd(true, true);
    let u = svd.u.unwrap();
    let s = svd.singular_values;
    let v_t = svd.v_t.unwrap();

    let multiplier = match quality {
        Quality::Low => 10,
        Quality::Medium => 50,
        Quality::High => 100,
    };

    let s_diag = DMatrix::from_diagonal(&s.rows(0, multiplier));
    let compressed_img_mtx = u.columns(0, multiplier) * s_diag * v_t.rows(0, multiplier);
    let compressed_img = matrix_to_dynamic_image(&compressed_img_mtx);
    compressed_img.save("./test/compressed_cat.jpeg").unwrap();
}

// Convert a matrix to a dynamic image
fn matrix_to_dynamic_image(
    matrix: &Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>>,
) -> DynamicImage {
    // Assuming the matrix represents grayscale image data
    let rows = matrix.nrows() as u32;
    let cols = matrix.ncols() as u32;

    // Create a grayscale image from the matrix
    let mut gray_image = GrayImage::new(cols, rows);

    // Iterate over the matrix elements and set pixel values in the image
    for y in 0..rows {
        for x in 0..cols {
            let pixel_value = matrix[(y as usize, x as usize)]; // Convert pixel value to u8
            gray_image.put_pixel(x, y, Luma([pixel_value as u8]));
        }
    }

    // Convert the grayscale image to a dynamic image
    DynamicImage::ImageLuma8(gray_image)
}
