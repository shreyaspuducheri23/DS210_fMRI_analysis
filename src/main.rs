mod preprocessing;

use crate::preprocessing::*;

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
    let matrix = read_data::connectivity_matrix(r"data\case 1\ADHD200_CC200_KKI_1842819_connectmat.txt");
    
    let rows = matrix.len();
    let columns = matrix[0].len();

    assert_eq!(rows, 190);
    assert_eq!(columns, 190);
}
fn main() {
    let conn_matrix = read_data::connectivity_matrix(r"data\case 1\ADHD200_CC200_KKI_1842819_connectmat.txt");


}
