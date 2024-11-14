#![procedural::magic_macro]
//! Define a Matrix type as a 2x2 tuple
//!
//! Then define a function called transpose that transpose it
//!

type Matrix = ((i32, i32), (i32, i32));

fn transpose(matrix: Matrix) -> Matrix {
    let mut trans = matrix;
    let tmp = trans.0 .1;
    trans.0 .1 = trans.1 .0;
    trans.1 .0 = tmp;

    trans
}

#[runtest(1.0, Matrix)]
/// Test if matrix is correctly defined as a double tuple o i32
fn test_if_matrix_correctly_defined() {
    let t: Matrix = ((0i32, 0i32), (0i32, 0i32));
}

#[runtest(1.0, transpose)]
/// Write a function that takes a "matrix" (2x2, i32 tuple) as input, transposes and returns it.
fn test_transpose() {
    let matrix: Matrix = ((1, 2), (3, 4));
    //println!("{:?}\n{:?}", matrix.0, matrix.1);

    let trans = transpose(matrix);
    //println!("Transpose \n {:?}\n{:?}", trans.0, trans.1);
    assert_eq!(trans, ((1, 3), (2, 4)));
}
