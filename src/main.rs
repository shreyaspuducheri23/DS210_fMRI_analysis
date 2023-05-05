mod preprocessing;
mod clustering;
use crate::preprocessing::*;
use crate::clustering::*;
use crate::preprocessing::read_data::Node;

use std::fs;
use std::io;
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
    //to remove previous images before creating new ones
    let folder_path = "images";

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

    let adj_matrix_6_cases = vec![
        read_data::read_connectivity_matrix(r"data\case 1\ADHD200_CC200_KKI_1842819_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 2\ADHD200_CC200_KKI_2026113_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 3\ADHD200_CC200_KKI_3434578_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 4\ADHD200_CC200_KKI_8628223_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 5\ADHD200_CC200_KKI_1623716_connectmat.txt"),
        read_data::read_connectivity_matrix(r"data\case 6\ADHD200_CC200_KKI_1594156_connectmat.txt")
    ];

    let mut graph = read_data::fMRI_graph {
        nodes: read_data::read_node_names(r"data\case 1\ADHD200_CC200_KKI_1842819_region_names.txt"),
        adjacency_matrix: read_data::average_matrices(&adj_matrix_6_cases),
    };
    read_data::read_node_centers(r"data\case 1\ADHD200_CC200_KKI_1842819_region_xyz_centers.txt", &mut graph);
    
    let mut Clusters: Vec<Vec<read_data::Node>> = graph.nodes.iter().map(|item| vec![item.clone()]).collect();
    while Clusters.len() != 8 {
        agglomerative_clustering::merge(& mut Clusters, &graph);
        
        if Clusters.len() == 8 {
            visualization::plot_clusters(&Clusters, &graph);
            for (i, cluster) in Clusters.iter().enumerate() {
                println!("Region {}:", i + 1);
                println!("{:?}", cluster.iter().map(|node| node.name.clone()).collect::<Vec<String>>());
                println!("");
            }
        }
    }
    
    
}
