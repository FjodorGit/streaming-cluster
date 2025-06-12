use criterion::{Criterion, criterion_group, criterion_main};
use streaming_cluster::example::generate_cloud;

fn execute_clustering() {
    let to_cluster = generate_cloud(500, 500);
    let mut streaming_cluster = streaming_cluster::StreamingCluster::new(1000);
    for point in to_cluster {
        streaming_cluster.add(point);
    }
    streaming_cluster.cluster_points();
}

pub fn streaming_cluster_benchmark(c: &mut Criterion) {
    c.bench_function("streaming 500 10000", |b| b.iter(|| execute_clustering()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = streaming_cluster_benchmark
}
criterion_main!(benches);
