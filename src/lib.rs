#[cfg(feature = "example")]
pub mod example;

use core::f32;

pub trait Metrizable {
    type Representer;
    fn representation(&self) -> Self::Representer;
    fn distance(some: &Self::Representer, other: &Self::Representer) -> f32;
}

struct ClusterPoint<T: Metrizable> {
    item: T,
    repr: T::Representer,
    weight: usize,
}

impl<T: Metrizable> ClusterPoint<T> {
    pub fn new(item: T, repr: T::Representer) -> Self {
        Self {
            item,
            repr,
            weight: 1,
        }
    }

    pub fn item(self) -> T {
        self.item
    }

    pub fn add_point(&mut self) {
        self.weight += 1;
    }

    pub fn merge_point(&mut self, other_cluster: &ClusterPoint<T>) {
        self.weight += other_cluster.weight;
    }

    pub fn repr(&self) -> &T::Representer {
        &self.repr
    }
}

pub struct StreamingCluster<T: Metrizable> {
    initialization_phase: bool,
    max_cluster_count: usize,
    clusters_centers: Vec<ClusterPoint<T>>,
    phi: f32,
}

impl<T: Metrizable> StreamingCluster<T> {
    pub fn new(max_cluster_count: usize) -> Self {
        Self {
            initialization_phase: true,
            max_cluster_count,
            clusters_centers: vec![],
            phi: f32::MAX,
        }
    }

    pub fn cluster_points(self) -> Vec<T> {
        self.clusters_centers
            .into_iter()
            .map(|cl| cl.item())
            .collect()
    }

    pub fn add(&mut self, item: T) {
        let repr = item.representation();
        if self.initialization_phase && self.clusters_centers.len() > self.max_cluster_count {
            self.initialization_phase = false;
        }

        if self.initialization_phase {
            if let Some(min_dist) = self.min_distance(&repr) {
                if min_dist < self.phi {
                    self.phi = min_dist;
                }
            }
            self.clusters_centers.push(ClusterPoint::new(item, repr));
            return;
        }

        let (closest_center, closest_center_distance) = self
            .clusters_centers
            .iter_mut()
            .map(|center| {
                let dist = T::distance(center.repr(), &repr);
                (center, dist)
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .expect("safe to unwrap because of the previous check");

        if closest_center_distance <= 4. * self.phi {
            closest_center.add_point();
        } else {
            self.clusters_centers.push(ClusterPoint::new(item, repr));
        }
        self.merge_rule();
    }

    fn merge_rule(&mut self) {
        if self.clusters_centers.len() <= self.max_cluster_count {
            return;
        }
        self.phi *= 2.;
        let mut merged_centers = vec![];
        while let Some(mut center_to_check) = self.clusters_centers.pop() {
            self.clusters_centers.retain(|other_center| {
                let distance = T::distance(center_to_check.repr(), other_center.repr());
                if distance > 4. * self.phi {
                    return true;
                } else {
                    center_to_check.merge_point(other_center);
                    return false;
                }
            });
            merged_centers.push(center_to_check);
        }
        self.clusters_centers.extend(merged_centers.drain(..));
    }

    fn min_distance(&self, repr: &T::Representer) -> Option<f32> {
        self.clusters_centers
            .iter()
            .map(|cluster_point| T::distance(cluster_point.repr(), repr))
            .reduce(f32::min)
    }
}
