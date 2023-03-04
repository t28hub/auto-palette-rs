use crate::math::clustering::dbscan::algorithm::DBSCAN;
use crate::math::clustering::dbscan::params::Params;
use crate::math::clustering::traits::Fit;
use crate::math::distance::euclidean::SquaredEuclideanDistance;
use crate::math::point::Point5;
use crate::swatch::Swatch;
use num_traits::ToPrimitive;

pub struct ImageData<'a> {
    data: &'a [u8],
    pub width: u32,
    pub height: u32,
}

impl<'a> ImageData<'a> {
    pub fn new(data: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn extract(&self) -> Vec<Swatch> {
        let width_u64 = self.width.to_u64().expect("Width could not be cast to u64");
        let height_u64 = self
            .height
            .to_u64()
            .expect("Height could not be cast to u64");

        let width_f64 = self.width.to_f64().expect("Width could not be cast to f64");
        let height_f64 = self
            .height
            .to_f64()
            .expect("Height could not be cast to f64");

        let mut index = 0;
        let mut pixels = Vec::with_capacity(self.data.len() / 4);
        while index < self.data.len() {
            let r = f64::from(self.data[index]) / 255.0;
            let g = f64::from(self.data[index + 1]) / 255.0;
            let b = f64::from(self.data[index + 2]) / 255.0;

            let index_u64 = index.to_u64().expect("Index could not be cast to u64");
            let x = (index_u64 / 4 % width_u64)
                .to_f64()
                .expect("X-coordinate could not be cast to f64");
            let y = ((index_u64 / 4 / width_u64) % height_u64)
                .to_f64()
                .expect("Y-coordinate could not be cast to f64");
            pixels.push(Point5(r, g, b, x / width_f64, y / height_f64));
            index += 4;
        }

        let params = Params::new(9, 0.016, SquaredEuclideanDistance);
        let dbscan = DBSCAN::fit(&pixels, &params);

        let mut swatches: Vec<Swatch> = dbscan
            .centroids()
            .into_iter()
            .enumerate()
            .map(|(cluster_id, centroid)| {
                let color = (
                    (centroid[0] * 255.0) as u8,
                    (centroid[1] * 255.0) as u8,
                    (centroid[2] * 255.0) as u8,
                );
                let position = (
                    (centroid[3] * width_f64)
                        .to_u32()
                        .expect("Width should be converted to u32"),
                    (centroid[4] * height_f64)
                        .to_u32()
                        .expect("Height should be converted to u32"),
                );
                let count = dbscan.count_at(cluster_id);
                let percentage = f64::from(count as u32) / f64::from(pixels.len() as u32);
                Swatch {
                    color,
                    position,
                    percentage,
                }
            })
            .collect();

        swatches.sort();
        swatches
    }
}
