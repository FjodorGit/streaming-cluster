# streaming-cluster
Tiny (~100 loc), zero dependencies stream clustering library

Just provide a way to calculate distance between your structs and the library will calculate representative centers from a stream of data.
See the simple [example](https://github.com/FjodorGit/streaming-cluster/blob/main/tests/functionality_tests.rs).

Implementation of the streaming algorithm from [here](https://www.researchgate.net/publication/220617991_Incremental_Clustering_and_Dynamic_Information_Retrieval).
