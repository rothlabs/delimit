use super::*;

fn new_vectors() -> (Vector<f64>, Vector<f64>) {
    (Vector::from(vec![0., 1., 2.]), Vector::from(vec![7., 14., 20.]))
}

fn new_matrices() -> (Matrix<f64>, Matrix<f64>) {
    (Matrix::rows_data(4, vec![
        0., 1., 2., 3., 
        4., 5., 6., 7.,
        8., 9., 0., 1., 
        2., 3., 4., 5.,
    ]), Matrix::rows_data(4, vec![
        1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., 1., 0.,
        0., 0., 0., 1.,
    ]))
}

#[test]
fn vector_add() {
    let (v1, v2) = &new_vectors();
    assert_eq!((v1 + v2).vec(), &vec![7., 15., 22.]);
}

#[test]
fn vector_subtract() {
    let (v1, v2) = &new_vectors();
    assert_eq!((v1 - v2).vec(), &vec![-7., -13., -18.]);
}

#[test]
fn vector_multiply() {
    let (v1, v2) = &new_vectors();
    assert_eq!((v1 * v2).vec(), &vec![0., 14., 40.]);
}

#[test]
fn vector_dot_product() {
    let (v1, v2) = &new_vectors();
    assert_eq!(v1.dot(v2), 54.);
}

#[test]
fn matrix_transpose() {
    let (m1, _) = &new_matrices();
    assert_eq!(m1.transpose().vec(), &vec![
        0., 4., 8., 2., 
        1., 5., 9., 3.,
        2., 6., 0., 4., 
        3., 7., 1., 5.,
    ]);
}

#[test]
fn matrix_multiply() {
    let (m1, m2) = &new_matrices();
    assert_eq!((m1 * m2).vec(), &vec![
        0., 1., 2., 3., 
        4., 5., 6., 7.,
        8., 9., 0., 1., 
        2., 3., 4., 5.,
    ]);
}
