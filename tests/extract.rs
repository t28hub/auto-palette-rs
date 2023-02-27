extern crate image;

use auto_palette_rs::image::ImageData;

#[test]
fn extract() {
    let img = image::open("./tests/images/flag_gr.png").unwrap();
    let data = img.to_rgba8().to_vec();
    let image_data = ImageData::new(&data, img.width(), img.height());
    let swatches = image_data.extract();
    println!("Swatches: {:?}", swatches);
    assert_eq!(swatches.len(), 2);
}
