use super::*;

#[test]
fn add() {
    #[rustfmt::skip]
    let base = Stem::new(Bare::Array(Array::new([4, 4, 1], vec![
        1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., 1., 0.,
        0., 0., 0., 1.,
    ])));
    let vector = Vector::new([4], vec![1., 2., 3., 4.]);
    let add = Add::new(&base, &vector);
    add.link.solve(Task::Array).read(|bare| {
        #[rustfmt::skip]
        assert_eq!(bare.array_ref().vec(), &vec![
            2., 2., 3., 4.,
            1., 3., 3., 4.,
            1., 2., 4., 4.,
            1., 2., 3., 5.,
        ]);
    });
}

// let out = add.link.solve(Task::Array).load().array();
// #[rustfmt::skip]
// assert_eq!(out.vec(), &vec![
//     2., 2., 3., 4.,
//     1., 3., 3., 4.,
//     1., 2., 4., 4.,
//     1., 2., 3., 5.,
// ]);
