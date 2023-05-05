pub mod read_data {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use regex::Regex;
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub struct Node {
        pub node_idx: usize,
        pub name: String,
        //pub betweenness: Option<f32>,
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

    pub fn read_connectivity_matrix(path: &str) -> Vec<Vec<f64>>{
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
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut names_vec: Vec<Node> = vec![];
        for (idx, line) in reader.lines().enumerate() {
            let node = Node {
                node_idx: idx,
                name: line.unwrap(),
                //betweenness: None,
                loc: None,
            };
            names_vec.push(node);
        }
        return names_vec;
    }

    pub fn read_node_centers (path: &str, graph: &mut fMRI_graph) {
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
        let mut average_matrix: Vec<Vec<f64>> = vec![vec![0.0; 190]; 190];

        for matrix in matrices {
            for i in 0..190 {
                for j in 0..190 {average_matrix[i][j] += matrix[i][j];}
            }
        }

        for i in 0..190 {
            for j in 0..190 {
                average_matrix[i][j] /= (matrices.len() as f64);
            }
        }

        return average_matrix;
    }
    
}