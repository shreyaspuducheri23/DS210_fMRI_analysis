pub mod read_data {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;
    use regex::Regex;

    pub fn connectivity_matrix(path: &str) -> Vec<Vec<f64>>{
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
}