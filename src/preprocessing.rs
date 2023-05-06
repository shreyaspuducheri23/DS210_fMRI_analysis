pub mod read_data {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use regex::Regex;
    #[derive(Debug, PartialEq, Clone)]
    pub struct Node {
        pub node_idx: usize, //The index of the node is necessary to index into the adjacency matrix
        pub name: String,
        pub loc: Option<(f64,f64,f64)>,
    }
    #[derive(Debug)]
    pub struct fMRI_graph {
        pub nodes: Vec<Node>,
        pub adjacency_matrix: Vec<Vec<f64>>,
    }

    impl fMRI_graph {
        pub fn get_adjacency_matrix(&self) -> &Vec<Vec<f64>> {
            &self.adjacency_matrix
        }
    }

    impl Node {
        pub fn find_parent_cluster(&self, clusters: &Vec<Vec<Node>>) -> Option<usize> {
            // Identifies the index of the cluster to which the node belongs
            let mut parent_cluster_idx: Option<usize> = None;
            while parent_cluster_idx == None {
                for (idx, cluster) in clusters.iter().enumerate() {
                    if cluster.contains(&self) {
                        parent_cluster_idx = Some(idx);
                    }
                }
            }
            return parent_cluster_idx;
        }
    }

    pub fn read_connectivity_matrix(path: &str) -> Vec<Vec<f64>>{
        //reads a connectivity matrix at the specified filepath and returns an 2D vec
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut conn_matrix: Vec<Vec<f64>> = vec![];
    
        for line in reader.lines() {
            let line = line.unwrap();
            let str_edges: Vec<&str> = line.split_whitespace().collect();
            let mut edges  = vec![]; 
            for edge in str_edges {
                edges.push(sci_to_float(edge));
            }
            conn_matrix.push(edges);
        }
        conn_matrix
    }
    pub fn sci_to_float(scientific: &str) -> f64 {
        // the edge weights are represented in scientific notation
        // this function reads a number in scientific notation, and returns it as an f64
        let pattern = Regex::new(r"(\d+(\.\d+)?)e([+-]?\d+)").unwrap();
        let mut num: f64 = 0.0;
        let mut exp: f64 = 0.0;
        if let Some(capture) = pattern.captures(scientific) {
            num = capture[1].parse().unwrap();
            exp = capture[capture.len() -1].parse().unwrap();
        }
        return num * 10_f64.powf(exp);
    }
    pub fn read_node_names(path: &str) -> Vec<Node> {
        //reading the names of each brain region and returning a vector of vertex names
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut names_vec: Vec<Node> = vec![];
        for (idx, line) in reader.lines().enumerate() {
            let node = Node {
                node_idx: idx,
                name: line.unwrap(),
                loc: None,
            };
            names_vec.push(node);
        }
        return names_vec;
    }

    pub fn read_node_centers (path: &str, graph: &mut fMRI_graph) {
        //reads the (x,y,z) coordinates of each brain region and updates the fMRI_graph data structure with this info
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for (idx, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let coordinates_vec: Vec<f64> = line.split_whitespace().map(|num| num.parse::<f64>().expect("Failed to parse the string as a float")).collect();
            //making sure the coordinates are valid
            if coordinates_vec.len() != 3 {panic!("Input string does not contain 3 coordinates");}
            let coordinates: (f64, f64, f64) = (coordinates_vec[0], coordinates_vec[1], coordinates_vec[2]);
            graph.nodes[idx].loc = Some(coordinates);
        }
    }

    pub fn average_matrices(matrices: &Vec<Vec<Vec<f64>>>) -> Vec<Vec<f64>> {
        // averages multiple 2d vectors and returns this average 2d vec
        let mut average_matrix: Vec<Vec<f64>> = vec![vec![0.0; matrices[0].len()]; matrices[0].len()];

        for matrix in matrices {
            for i in 0..matrices[0].len() {
                for j in 0..matrices[0].len() {average_matrix[i][j] += matrix[i][j];}
            }
        }

        for i in 0..matrices[0].len() {
            for j in 0..matrices[0].len() {
                average_matrix[i][j] /= (matrices.len() as f64);
            }
        }

        return average_matrix;
    }
    
}