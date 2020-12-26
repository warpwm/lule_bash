extern crate image;
extern crate pigmnts;

use pigmnts::{Pixels, color::LAB, weights, pigments_pixels};
use image::GenericImageView;

fn pigments(image_path: &str, count: u8) -> Result<Vec<(LAB, f32)>, Box<dyn std::error::Error>> {
    let mut img;

    img = image::open(image_path)?;
    img = img.resize(512, 512, image::imageops::FilterType::CatmullRom);

    let pixels: Pixels = img
        .pixels()
        .map(|(_, _, pix)| LAB::from_rgb(pix[0], pix[1], pix[2]))
        .collect();

    let weightfn = weights::resolve_mood(&weights::Mood::Dominant);
    let mut output = pigments_pixels(&pixels, count, weightfn, None);

    output.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    return Ok(output);
}

pub fn colors(image_path: &str, count: u8) -> Vec<String> {
    let result = pigments(image_path, count)
        .unwrap_or_else(|err| {
            eprintln!("Problem creating palette: {}", err);
            std::process::exit(1);
        });

    let mut record = Vec::new();

    for (color, _) in result.iter() {
        record.push(format!("{}", color));
    }
    record
}
