mod preprocessing;
mod clustering;
use crate::preprocessing::*;
use crate::clustering::*;

#[test]
fn test_sci_to_float() {
    // Test case 1 with positive exp
    let result = read_data::sci_to_float("1.23e2");
    assert_eq!(result, 123.0);

    // Test case 2 with negative exp
    let result = read_data::sci_to_float("5.5678e-3");
    assert_eq!(result, 0.0055678);
}

#[test]
fn test_dimensions_of_conn_matrix() {
    // dimensions of this matrix should be 190 x 190
    let matrix = read_data::read_connectivity_matrix(r"data\case 1\ADHD200_CC200_KKI_1842819_connectmat.txt");
    
    let rows = matrix.len();
    let columns = matrix[0].len();

    assert_eq!(rows, 190);
    assert_eq!(columns, 190);
}
fn main() {
    /*
    let graph = read_data::fMRI_graph {
        nodes: read_data::read_node_names(r"data\case 1\ADHD200_CC200_KKI_1842819_region_names.txt"),
        adjacency_matrix: read_data::read_connectivity_matrix(r"data\case 1\ADHD200_CC200_KKI_1842819_connectmat.txt"),
    };
    let mut Clusters: Vec<Vec<read_data::Node>> = graph.nodes.iter().map(|item| vec![item.clone()]).collect();
    while Clusters.len() != 8 {
        //println!("{:?}", Clusters);
        //println!("");
        //println!("");
        agglomerative_clustering::merge(& mut Clusters, &graph);
    }
    */
    let data=vec![(0.3,2.5,5.4), (4.6,2.4,9.2)];
    visualization::figure_3D(data)
}
