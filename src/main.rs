mod preprocessing;
mod clustering;
use crate::preprocessing::*;
use crate::clustering::*;
use crate::preprocessing::read_data::Node;

use std::fs;
use std::io;
use std::fs::File;
use std::io::Write;

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

#[test]
fn test_average_matrices() {
    let matrices = vec![
        vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ],
        vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ],
    ];
    let expected_result = vec![
        vec![3.0, 4.0],
        vec![5.0, 6.0],
    ];
    let result = read_data::average_matrices(&matrices);
    assert_eq!(result, expected_result);
}

#[test]
fn test_find_parent_cluster() {
    let node1 = Node {
        node_idx: 0,
        name: String::from("Node 1"),
        loc: None,
    };
    let node2 = Node {
        node_idx: 1,
        name: String::from("Node 2"),
        loc: None,
    };
    let node3 = Node {
        node_idx: 2,
        name: String::from("Node 3"),
        loc: None,
    };
    let node4 = Node {
        node_idx: 3,
        name: String::from("Node 4"),
        loc: None,
    };
    let clusters = vec![
        vec![node1.clone(), node2.clone()],
        vec![node3.clone(), node4.clone()],
    ];

    // Test that the function correctly identifies the parent cluster for a node
    assert_eq!(node1.find_parent_cluster(&clusters), Some(0));
    assert_eq!(node2.find_parent_cluster(&clusters), Some(0));
    assert_eq!(node3.find_parent_cluster(&clusters), Some(1));
}

#[test]
fn test_merge() {
    use std::collections::HashSet;
    
    let node1 = Node {
        node_idx: 0,
        name: String::from("Node 1"),
        loc: None,
    };
    let node2 = Node {
        node_idx: 1,
        name: String::from("Node 2"),
        loc: None,
    };
    let node3 = Node {
        node_idx: 2,
        name: String::from("Node 3"),
        loc: None,
    };
    let node4 = Node {
        node_idx: 3,
        name: String::from("Node 4"),
        loc: None,
    };
    let mut clusters = vec![vec![node1.clone(), node2.clone()], vec![node3.clone()], vec![node4.clone()]];
    let graph = read_data::fMRI_graph {
        nodes: vec![node1.clone(), node2.clone(), node3.clone(), node4.clone()],
        adjacency_matrix: vec![
            vec![0.0, 1.0, 1.0, 0.0], 
            vec![1.0, 0.0, 1.0, 0.0], 
            vec![1.0, 1.0, 0.0, 0.5], 
            vec![0.0, 0.0, 0.5, 0.0]
        ],
    };

    // Test that the function correctly merges two clusters
    agglomerative_clustering::merge(&mut clusters, &graph);
    
    // Create a HashSet of node names
    let expected_node_names = vec!["Node 1", "Node 2", "Node 3"]
    .into_iter()
    .collect::<std::collections::HashSet<&str>>();

    // Get the node names from the graph
    let node_names = clusters[0]
        .iter()
        .map(|node| node.name.as_str())
        .collect::<std::collections::HashSet<&str>>();

    // Assert that the node names are correct
    assert_eq!(node_names, expected_node_names);
}

fn main() {
    //to remove previous images before creating new ones
    let folder_path = "results/images";

    if let Err(err) = fs::remove_dir_all(folder_path) {
        eprintln!("Error deleting folder contents: {}", err);
    } else {
        println!("Folder contents deleted successfully!");
    }
    if let Err(err) = fs::create_dir(folder_path) {
        eprintln!("Error creating folder: {}", err);
    } else {
        println!("Folder created successfully!");
    }
    //reading 6 fMRI adjacency matrices
    let adj_matrix_6_cases = vec![
        read_data::read_connectivity_matrix(r"data\case 1\ADHD200_CC200_KKI_1842819_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 2\ADHD200_CC200_KKI_2026113_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 3\ADHD200_CC200_KKI_3434578_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 4\ADHD200_CC200_KKI_8628223_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 5\ADHD200_CC200_KKI_1623716_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 6\ADHD200_CC200_KKI_1594156_connectmat.txt")
    ];

    
    //implimentation of the fMRI data structure
    let mut graph = read_data::fMRI_graph {
        nodes: read_data::read_node_names(r"data\case 1\ADHD200_CC200_KKI_1842819_region_names.txt"),
        adjacency_matrix: read_data::average_matrices(&adj_matrix_6_cases), //consolidating the 6 fMRIs into one average connectivity matrix
    };
    read_data::read_node_centers(r"data\case 1\ADHD200_CC200_KKI_1842819_region_xyz_centers.txt", &mut graph);
    
    let mut Clusters: Vec<Vec<read_data::Node>> = graph.nodes.iter().map(|item| vec![item.clone()]).collect(); // the initial clusters should just be the unique nodes
    while Clusters.len() != 8 {//8 clusters yields regions consistent with literature
        agglomerative_clustering::merge(& mut Clusters, &graph);
    }
    visualization::plot_clusters(&Clusters, &graph);
    let mut file = File::create("results/brain_regions.txt").unwrap();
    for (i, cluster) in Clusters.iter().enumerate() {
        writeln!(file, "Region {}:", i + 1);
        writeln!(file, "{:?}", cluster.iter().map(|node| node.name.clone()).collect::<Vec<String>>());
        writeln!(file, "");
    }
    
}
