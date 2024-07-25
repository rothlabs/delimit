use super::*;

#[test]
fn add_arrays() {
    #[rustfmt::skip]
    let base = Stem::new(Bare::Mem(Array::new([4, 4, 1], vec![
        1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., 1., 0.,
        0., 0., 0., 1.,
    ])));
    let vector = Array1::new([4], vec![1., 2., 3., 4.]);
    let add = Add::hold(&base, &vector);
    add.link.grant().read(|bare| {
        #[rustfmt::skip]
        assert_eq!(bare.array_ref().vec(), &vec![
            2., 2., 3., 4.,
            1., 3., 3., 4.,
            1., 2., 4., 4.,
            1., 2., 3., 5.,
        ]);
    });
}
