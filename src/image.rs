use crate::math::clustering::kmeans::algorithm::Kmeans;
use crate::math::clustering::kmeans::init::Initializer::KmeansPlusPlus;
use crate::math::clustering::kmeans::params::KmeansParams;
use crate::math::clustering::traits::Fit;
use crate::math::distance::euclidean::SquaredEuclideanDistance;
use crate::math::point::Point3;
use crate::swatch::Swatch;
use rand::thread_rng;

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
            let pixel = Point3::<f64>::new(
                self.data[index].into(),
                self.data[index + 1].into(),
                self.data[index + 2].into(),
            );
            pixels.push(pixel);
            index += 4;
        }

        let params = KmeansParams::new(
            2,
            SquaredEuclideanDistance::default(),
            KmeansPlusPlus(thread_rng()),
        )
        .with_max_iterations(100)
        .with_tolerance(0.0001);
        let kmeans = Kmeans::fit(&pixels, &params);
        kmeans
            .centroids()
            .iter()
            .enumerate()
            .map(|(cluster_id, centroid)| {
                let rgb = centroid.to_vec();
                let color = (rgb[0] as u8, rgb[1] as u8, rgb[2] as u8);
                let count = kmeans.count_at(cluster_id);
                let percentage = f64::from(count as u32) / f64::from(pixels.len() as u32);
                Swatch { color, percentage }
            })
            .collect()
    }
}
