use streaming_cluster::{
    StreamingCluster,
    example::{ThreeDimVec, generate_cloud},
};

#[test]
fn cluster_creation_of_three_dim_vec() {
    let to_cluster = generate_cloud(5, 20);
    let mut streaming_cluster = StreamingCluster::<ThreeDimVec>::new(10);
    for point in to_cluster {
        streaming_cluster.add(point);
    }

    let cluster_points = streaming_cluster.cluster_points();
    assert_eq!(cluster_points.len(), 5);
}
