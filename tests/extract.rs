extern crate image;

use auto_palette_rs::image::ImageData;
use auto_palette_rs::swatch::Swatch;

#[test]
fn extract() {
    let img = image::open("./tests/images/flag_gr.png").unwrap();
    let data = img.to_rgba8().to_vec();
    let image_data = ImageData::new(&data, img.width(), img.height());
    let swatches: Vec<Swatch> = image_data
        .extract()
        .into_iter()
        .filter(|swatch| swatch.percentage > 0.05)
        .collect();
    assert_eq!(swatches.len(), 2);
}
