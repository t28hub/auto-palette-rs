use crate::math::clustering::dbscan::algorithm::DBSCAN;
use crate::math::clustering::dbscan::params::Params;
use crate::math::clustering::traits::Fit;
use crate::math::distance::euclidean::SquaredEuclideanDistance;
use crate::math::point::{Point, Point3};
use crate::swatch::Swatch;

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
        let mut index = 0;
        let mut pixels = Vec::with_capacity(self.data.len() / 4);
        while index < self.data.len() {
            let pixel = Point3::new(
                self.data[index].into(),
                self.data[index + 1].into(),
                self.data[index + 2].into(),
            );
            pixels.push(pixel);
            index += 4;
        }

        let params = Params::new(9, 8.0, SquaredEuclideanDistance);
        let dbscan = DBSCAN::fit(&pixels, &params);
        println!("{:?}", dbscan.outliers());
        dbscan
            .centroids()
            .iter()
            .enumerate()
            .map(|(cluster_id, centroid)| {
                let rgb = centroid.to_vec();
                let color = (rgb[0] as u8, rgb[1] as u8, rgb[2] as u8);
                let count = dbscan.count_at(cluster_id);
                let percentage = f64::from(count as u32) / f64::from(pixels.len() as u32);
                Swatch { color, percentage }
            })
            .collect()
    }
}
