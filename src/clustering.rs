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

    pub fn cluster_correlation(cluster_1: &Vec<Node>, cluster_2: &Vec<Node>, graph: &fMRI_graph) -> f64 {
        // This function finds the average correlation between two clusters
        let mut avg_corr: f64 = 0.0;
        for node_a in cluster_1 {
            for node_b in cluster_2 {
                avg_corr += graph.adjacency_matrix[node_a.node_idx][node_b.node_idx]
            }
        }
        avg_corr = avg_corr / (cluster_1.len() * cluster_2.len()) as f64;
        return avg_corr;
    }

    pub fn cluster_neighbor(clusters: &Vec<Vec<Node>>, graph: &fMRI_graph) -> (usize, usize) {
        // This function finds the cluster with maximum avg correlation to any other cluster
        let (mut best_corr, mut best_cluster_1_idx, mut best_cluster_2_idx): (f64, Option<usize>, Option<usize>) = (0.0, None, None);
        for i in 0..clusters.len() {
            for j in 0..i {
                let avg_corr = cluster_correlation(&clusters[i], &clusters[j], graph);
                if  avg_corr > best_corr {
                    (best_corr, best_cluster_1_idx, best_cluster_2_idx) = (avg_corr, Some(i), Some(j));
                }
            }
        }
        return (best_cluster_1_idx.unwrap(), best_cluster_2_idx.unwrap());
    }

    pub fn merge (clusters: &mut Vec<Vec<Node>>, graph: &fMRI_graph) {
        let (cluster_1_idx, cluster_2_idx): (usize, usize) = cluster_neighbor(clusters, graph);
        let mut cluster_to_merge: Vec<Node> = clusters[cluster_2_idx].clone();
        clusters[cluster_1_idx].append(& mut cluster_to_merge);
        clusters.remove(cluster_2_idx);
    }
}

pub mod visualization {
    use plotters::data;
    use plotters::prelude::*;
    use crate::preprocessing::read_data::{fMRI_graph, Node};

    pub fn plot_clusters(clusters: &Vec<Vec<Node>>, graph: &fMRI_graph) {
        let mut series: Vec<Vec<(f64, f64, f64)>> = vec![];
        for cluster in clusters {
            let mut locs_of_cluster: Vec<(f64, f64, f64)> = cluster.iter().filter_map(|node| node.loc).collect();
            series.push(locs_of_cluster);
        }
        
        figure_3D(series);
    }

    pub fn figure_3D(data: Vec<Vec<(f64, f64, f64)>>) {
        figure_3D_perspective(&data, 0.7, 0.0, "_front");
        figure_3D_perspective(&data, 0.7, 0.9, "_right");
        figure_3D_perspective(&data, 0.7, -0.9, "_left");
        figure_3D_perspective(&data, -0.7, 0.0, "_rear");
    }
    
    pub fn figure_3D_perspective(data: &Vec<Vec<(f64,f64,f64)>>, pitch: f64, yaw: f64, view_name: &str) {
        //takes a vec of vec of the xyz coordinates and plots them in three dimensions, with different colors to represent each brain region
        let binding = ("images/".to_owned() + &data.len().to_string() + view_name + &".png");
        let root = BitMapBackend::new(&binding, (640, 480)).into_drawing_area();
        let mut chart = ChartBuilder::on(&root)
                .margin(20)
                .caption("Brain regions", ("sans-serif", 40))
                .build_cartesian_3d::<std::ops::Range<f64>, std::ops::Range<f64>, std::ops::Range<f64>>(-100.0..100.0, -100.0..100.0, -100.0..100.0)
                .unwrap();

        
        chart.with_projection(|mut pb| {
                pb.pitch = pitch;
                pb.yaw = yaw;
                pb.scale = 0.7;
                pb.into_matrix()
            });
        
        chart.plotting_area().fill(&WHITE).unwrap();
        chart.configure_axes().draw().unwrap();
        for (i, series) in data.iter().enumerate() {
            chart.draw_series(series.iter().map(|point| Circle::new(*point, 5, RGBColor((i as u8)*50,(i as u8)*50,(i as u8)*100).filled())))
                .unwrap();
        }
        let labels = vec![
            (String::from("Brain Stem"), (-3.585366, -24.195122, -36.878049 )),
            (String::from("Right Frontal Pole"), (49.311111, 36.955556, -10.844444)),
            (String::from("Left Frontal Pole"), (-31.920530, 57.615894, -12.437086)),
            (String::from("Right Temporal Pole"), (49.806452, 12.309677, -31.496774)),
            (String::from("Left Temporal Pole"), (-47.246377, 6.144928, -38.231884)),
            (String::from("Cerebellum Vermis"), (-0.579186, -62.986425, -18.923077))
        ];
        chart.draw_series(
            labels
                .iter()
                .map(|(label, position)| {
                    Text::new(
                        label.clone(),
                        *position,
                        ("sans-serif", 10).into_font(),
                    )
                })
        ).unwrap();
    }
    /*
    pub fn figure_3D(data: Vec<Vec<(f64, f64, f64)>>) {
        let binding = ("images/".to_owned() + &data.len().to_string() + &".png"); // Output directory path
        let image_width = 640;
        let image_height = 480;
        let num_views = 10; // Number of views to generate
    
        for view in 0..num_views {
            let angle = 360.0 * (view as f64) / (num_views as f64);
    
            let filename = format!("{}{}.png", binding, view);
            let root = BitMapBackend::new(&filename, (image_width, image_height)).into_drawing_area();
            root.fill(&WHITE).unwrap();
    
            let mut chart = ChartBuilder::on(&root)
                .margin(20)
                .caption("Brain regions", ("sans-serif", 40))
                .build_cartesian_3d::<std::ops::Range<f64>, std::ops::Range<f64>, std::ops::Range<f64>>(
                    -100.0..100.0,
                    -100.0..100.0,
                    -100.0..100.0,
                )
                .unwrap();
    
            // Configure and draw axes
            chart.configure_axes().draw().unwrap();
    
            for (i, series) in data.iter().enumerate() {
                chart
                    .draw_series(series.iter().map(|point| {
                        Circle::new(
                            rotate_point(*point, angle), // Rotate the point based on the angle
                            5,
                            RGBColor((i as u8) * 50, (i as u8) * 50, (i as u8) * 100).filled(),
                        )
                    }))
                    .unwrap();
            }
    
            // Save the image
            root.present().expect("Failed to save the image");
        }
    }
    
    fn rotate_point(point: (f64, f64, f64), angle: f64) -> (f64, f64, f64) {
        let angle_rad = angle * std::f64::consts::PI / 180.0;
        let (x, y, z) = point;
        let rotated_x = x * angle_rad.cos() - y * angle_rad.sin();
        let rotated_y = x * angle_rad.sin() + y * angle_rad.cos();
        (rotated_x, rotated_y, z)
    }
    */
}