use fft2d::slice::fft_2d;
use image::GenericImageView;
use rustfft::num_complex::Complex64;

// TODO: Write the inverse and decompress func
fn main() -> Result<(), std::io::Error> {
    let img = image::open("sample1.png").unwrap();
    // TODO: Use all color channels
    let mut pixel_complex: Vec<Complex64> = img
        .pixels()
        .map(|p| Complex64::new(p.2[0] as f64, 0.0))
        .collect();

    fft_2d(
        img.dimensions().0 as usize,
        img.dimensions().1 as usize,
        &mut pixel_complex,
    );

    let transformed: Vec<u8> = pixel_complex
        .clone()
        .iter()
        .flat_map(|p| vec![p.re as u8, p.re as u8, p.re as u8])
        .collect();
    let output_file = "output.png";

    image::save_buffer(
        output_file,
        &transformed,
        img.dimensions().0,
        img.dimensions().1,
        image::ColorType::Rgb8,
    )
    .unwrap();

    Ok(())
}
