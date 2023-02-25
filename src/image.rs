use crate::math::clustering::kmeans::algorithm::{Clustering, Kmeans};
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

        let distance = SquaredEuclideanDistance::default();
        let initializer = KmeansPlusPlus(thread_rng());
        let mut clustering = Kmeans::new(6, distance, initializer);
        clustering.fit(&pixels);
    }
}
