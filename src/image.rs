use crate::math::clustering::kmeans::algorithm::{Clustering, Kmeans, KmeansParams};
use crate::math::clustering::kmeans::initializer::CentroidInitializer::KmeansPlusPlus;
use crate::math::distance::euclidean::SquaredEuclideanDistance;
use crate::math::point::Point3;
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

    pub fn extract(&self) {
        let mut index = 0;
        let mut pixels = Vec::with_capacity(self.data.len() / 4);
        while index < self.data.len() {
            let pixel = Point3::<f32>::new(
                self.data[index].into(),
                self.data[index + 1].into(),
                self.data[index + 2].into(),
            );
            pixels.push(pixel);
            index += 4;
        }

        let mut params = KmeansParams::new(
            2,
            SquaredEuclideanDistance::default(),
            KmeansPlusPlus(thread_rng()),
        )
        .with_max_iterations(100)
        .with_tolerance(0.001);
        let kmeans = Kmeans::fit(&pixels, &mut params);
        println!("Kmeans: {:?}", kmeans.centroids())
    }
}
