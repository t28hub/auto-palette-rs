use crate::color::lab::Lab;
use crate::color::rgba::Rgba;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::math::clustering::dbscan::algorithm::DBSCAN;
use crate::math::clustering::dbscan::params::Params;
use crate::math::clustering::traits::Fit;
use crate::math::distance::euclidean::EuclideanDistance;
use crate::math::number::{Float, Number};
use crate::math::point::Point5;
use crate::swatch::Swatch;

pub struct ImageData<'a> {
    data: &'a [u8],
    pub width: u32,
    pub height: u32,
}

impl<'a> ImageData<'a> {
    #[must_use]
    pub fn new(data: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    #[must_use]
    pub fn extract<F: Float>(&self) -> Vec<Swatch<F>> {
        let width_u64: u64 = self.width();
        let height_u64: u64 = self.height();
        let width_f: F = self.width();
        let height_f: F = self.height();

        let delta_l: F = Lab::<F>::max_l::<F>() - Lab::<F>::min_l::<F>();
        let delta_a: F = Lab::<F>::max_a::<F>() - Lab::<F>::min_a::<F>();
        let delta_b: F = Lab::<F>::max_b::<F>() - Lab::<F>::min_b::<F>();

        let mut index = 0;
        let mut pixels = Vec::with_capacity(self.data.len() / 4);
        while index < self.data.len() {
            let rgba = Rgba::new(
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            );
            let xyz: XYZ<F, D65> = XYZ::from(&rgba);
            let Lab { l, a, b, .. } = Lab::from(&xyz);

            let index_u64 = u64::from_usize(index);
            let x = F::from_u64(index_u64 / 4 % width_u64);
            let y = F::from_u64((index_u64 / 4 / width_u64) % height_u64);
            // Normalize each value and convert as a point.
            pixels.push(Point5::new(
                l / delta_l,
                a / delta_a,
                b / delta_b,
                x / width_f,
                y / height_f,
            ));
            index += 4;
        }

        let params = Params::new(25, F::from_f64(0.025), EuclideanDistance);
        let dbscan = DBSCAN::fit(&pixels, &params);
        let mut swatches: Vec<Swatch<F>> = dbscan
            .centroids()
            .into_iter()
            .enumerate()
            .map(|(cluster_id, centroid)| {
                let lab = Lab::new(
                    centroid[0] * delta_l,
                    centroid[1] * delta_a,
                    centroid[2] * delta_b,
                );
                let xyz = XYZ::from(&lab);
                let rgb = Rgba::from(&xyz);
                let color = (rgb.r, rgb.g, rgb.b);

                let x = (centroid[3] * width_f)
                    .to_u32()
                    .expect("Width should be converted to u32");
                let y = (centroid[4] * height_f)
                    .to_u32()
                    .expect("Height should be converted to u32");
                let position = (x, y);

                let count = dbscan.count_at(cluster_id);
                let percentage = F::from_usize(count) / F::from_usize(pixels.len());
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

    fn width<N: Number>(&self) -> N {
        N::from_u32(self.width)
    }

    fn height<N: Number>(&self) -> N {
        N::from_u32(self.height)
    }
}
