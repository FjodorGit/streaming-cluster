use rand::{Rng, SeedableRng, seq::SliceRandom};
use rand_pcg::Pcg32;

use crate::Metrizable;

#[derive(Debug)]
pub struct ThreeDimVec(pub f32, pub f32, pub f32);

impl Metrizable for ThreeDimVec {
    type Representer = Vec<f32>;

    fn representation(&self) -> Self::Representer {
        vec![self.0, self.1, self.2]
    }

    fn distance(some: &Self::Representer, other: &Self::Representer) -> f32 {
        some.iter()
            .zip(other.iter())
            .map(|(s, o)| (s - o).abs().powf(2.))
            .sum()
    }
}

pub fn generate_cloud(centers_count: i32, points_per_cluster: usize) -> Vec<ThreeDimVec> {
    let start = -(centers_count / 2);
    let end_inclusive = if centers_count % 2 == 0 {
        (centers_count / 2) - 1
    } else {
        centers_count / 2
    };
    let centers: Vec<f32> = (start..=end_inclusive).map(|i| 3.0 * i as f32).collect();

    let mut rng = Pcg32::seed_from_u64(42);
    let mut to_cluster = vec![];
    for center in centers {
        to_cluster.push(ThreeDimVec(center, 0., 0.));
        for _ in 0..points_per_cluster {
            let y = rng.random_range(0.0..1.5) as f32;
            let z = rng.random_range(0.0..1.5) as f32;
            let three_dim_vec = ThreeDimVec(center, y, z);
            to_cluster.push(three_dim_vec);
        }
    }
    to_cluster.shuffle(&mut rng);
    to_cluster
}
