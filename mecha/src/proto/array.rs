pub type Array1<T> = Array<T, 1>;
// pub type Array2<T> = Array<T, 2>;
pub type Array3<T> = Array<T, 3>;

/// N-dimensional array
//#[derive(Clone)]
pub struct Array<T, const N: usize> {
    pub dims: [usize; N],
    jump: [usize; N],
    data: Vec<T>,
}

impl<T, const N: usize> Clone for Array<T, N>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        Self {
            dims: self.dims.clone(),
            jump: self.jump.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T, const N: usize> Array<T, N>
where
    T: Copy + Default,
{
    // pub fn new(dims: [usize; N]) -> Self {
    //     let (len, jump) = Self::meta(dims);
    //     Self {
    //         dims,
    //         jump,
    //         data: vec![T::default(); len],
    //     }
    // }
    pub fn new(dims: [usize; N], data: Vec<T>) -> Self {
        let (_, jump) = Self::meta(dims);
        Self { dims, jump, data }
    }
    pub fn get(&self, indices: [usize; N]) -> T {
        self.data[self.idx(indices)]
    }
    pub fn vec(&self) -> &Vec<T> {
        &self.data
    }
    pub fn each<F: Fn([usize; N], T) -> T>(&mut self, op: F) {
        for i in 0..self.data.len() {
            self.data[i] = op(self.indices(i), self.data[i]);
        }
    }
    fn meta(dims: [usize; N]) -> (usize, [usize; N]) {
        let mut jump = [0; N];
        let len = dims.iter().enumerate().fold(1, |a, x| {
            jump[x.0] = a;
            a * x.1
        });
        (len, jump)
    }
    fn idx(&self, indices: [usize; N]) -> usize {
        self.jump.iter().zip(indices).fold(0, |a, x| a + x.0 * x.1)
    }
    fn indices(&self, idx: usize) -> [usize; N] {
        let mut indices = [0; N];
        self.jump.iter().enumerate().rev().fold(idx, |a, x| {
            indices[x.0] = a / x.1;
            a % x.1
        });
        indices
    }
}

// pub fn get_mut<'a>(&'a mut self, indices: [usize; N]) -> &mut T {
//     let i = self.idx(indices);
//     &mut self.data[i]
// }

// pub fn each<F: FnMut(usize, usize)>(&self, op: &mut F) {
//     for n in 0..N {
//         for i in 0..self.dims[n] {
//             op(r, c);
//         }
//     }
// }
// fn indices(&self, idx: usize) -> [usize; N] {
//     self.dims.iter()
// }

// self.jump.iter().enumerate().rev().fold(0, |a, x| a + idx[x.0] * x.1)
