pub mod agglomerative_clustering {
    use crate::preprocessing::read_data::{fMRI_graph, Node};

    pub fn find_parent_cluster(clusters: &Vec<Vec<Node>>, child_node: &Node) -> Option<usize> {
        // Identifies the index of the cluster to which the child node belongs
        let mut parent_cluster_idx: Option<usize> = None;
        while parent_cluster_idx == None {
            for (idx, cluster) in clusters.iter().enumerate() {
                if cluster.contains(&child_node) {
                    parent_cluster_idx = Some(idx);
                }
            }
        }
        return parent_cluster_idx;
    }

    pub fn closest_neighbor(source_node: Node, clusters: &Vec<Vec<Node>>, graph: &fMRI_graph) -> (Option<Node>, f64) {
        // This function finds which node is most correlated with the source_node passed into the funciton using the adjacency matrix from the graph network
        let (mut max_corr, mut max_node): (f64, Option<Node>) = (0.0, None);
        for i in 0..graph.adjacency_matrix.len() {
            // The next most correlated vertex cannot be in the same cluster as the current vertex; that would be redundant, and the clusters would never grow iteratively
            if graph.adjacency_matrix[source_node.node_idx][i] > max_corr && !clusters[find_parent_cluster(clusters, &source_node).unwrap()].contains(&graph.nodes[i]) {
                (max_corr, max_node) = (graph.adjacency_matrix[source_node.node_idx][i], Some(graph.nodes[i].clone()));
            }
        }
        return (max_node, max_corr);
    }

    pub fn cluster_neighbor(clusters: &Vec<Vec<Node>>, graph: &fMRI_graph) -> (usize, Node) {
        // While each node will have a neighboring node that it is most correlated with, we are only interested in the node that each cluster is correlated with
        // This function finds the vertex with maximum correlation to any of the vertices in each cluster, then selects only the vertex with the maximum correlation
        let (mut best_neighbor, mut best_corr, mut best_cluster): (Option<Node>, f64, Option<usize>) = (None, 0.0, None);
        for (cluster_idx, cluster) in clusters.iter().enumerate() {
            let (mut best_cluster_neighbor, mut best_cluster_corr): (Option<Node>, f64) = (None, 0.0);
            for node in cluster {
                let (neighbor, corr) = closest_neighbor(node.clone(), clusters, graph);
                // Finding the best correlation to next vertex for each cluster
                if corr > best_cluster_corr {
                    (best_cluster_neighbor, best_cluster_corr) = (neighbor, corr)
                }
            }
            if best_cluster_corr >  best_corr {
                (best_cluster, best_neighbor, best_corr) = (Some(cluster_idx), best_cluster_neighbor, best_cluster_corr);
            }
        }
        return (best_cluster.unwrap(), best_neighbor.unwrap())
    }

    pub fn merge (clusters: &mut Vec<Vec<Node>>, graph: &fMRI_graph) {
        let (cluster_idx, node_to_merge): (usize, Node) = cluster_neighbor(clusters, graph);
        let cluster_idx_to_merge: usize = find_parent_cluster(clusters, &node_to_merge).unwrap();
        let cluster_to_merge = clusters[cluster_idx_to_merge].clone();
        clusters[cluster_idx].append(&mut cluster_to_merge.clone());
        clusters.remove(cluster_idx_to_merge);
    }
}

pub mod visualization {
    use plotters::data;
    use plotters::prelude::*;

    pub fn figure_3D(data: Vec<(f32,f32,f32)>) {
        let root = BitMapBackend::new("images/3d-env.png", (640, 480)).into_drawing_area();

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption("Empty 3D Figure", ("sans-serif", 40))
            .build_cartesian_3d(0.0..10.0, 0.0..10.0, 0.0..10.0)
            .unwrap();
        chart.configure_axes().draw().unwrap();

        chart.draw_series(data.iter().map(|point| Circle::new(*point, 5, &RED)))
            .unwrap();
    }
}