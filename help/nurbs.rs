pub fn knot_index(&self, u: f32) -> usize {
    for i in 0..self.knots.len() - 1 { 
        if u >= self.knots[i] && u < self.knots[i+1] { 
            return i;
        }
    }
    return self.knots.len() - self.order - 1;
}

pub fn plot(&self, u: f32) -> (usize, ([f32; 4], [f32; 4])) {
    let u = self.min * (1.-u) + self.max * u;
    let knot_index = self.knot_index(u);
    let mut basis = ([0., 0., 0., 1.], [0., 0., 0., 1.]);
    for degree in 1..self.order {
        for i in 0..degree + 1 { 
            let b0 = 3 - degree + i;
            let b1 = b0 + 1;
            let k0 = knot_index + i; 
            let k1 = k0 + 1;
            let mut position = 0.;
            let mut velocity = 0.;
            if basis.0[b0] > 0. { // piecewise part of b-spline basis N?
                let distance = self.knots[k0] - self.knots[k0 - degree];
                position += basis.0[b0] * (u - self.knots[k0 - degree]) / distance; // Part A of recursive N
                velocity += basis.0[b0] * degree as f32 / distance;
            }
            if b1 < 4 && basis.0[b1] > 0. { // piecewise part of b-spline basis N?
                let distance = self.knots[k1] - self.knots[k1 - degree];
                position += basis.0[b1] * (self.knots[k1] - u) / distance; // Part B of recursive N
                velocity -= basis.0[b1] * degree as f32 / distance;
            } 
            basis.0[b0] = position; 
            basis.1[b0] = velocity;
        }
    }
    let mut weights = [0., 0., 0., 0.];
    for i in 0..self.order {
        weights[i - self.order + 4] = self.weights[knot_index - self.order + 1 + i];
    }
    let mut sum = (0., 0.);
    for i in 0..4 {
        sum.0 += basis.0[i] * weights[i];
        sum.1 += basis.1[i] * weights[i];
    }
    for i in 0..4 {
        basis.1[i] = (basis.1[i] * sum.0 - basis.0[i] * sum.1) * weights[i] / sum.0 / sum.0;
        basis.0[i] *= weights[i] / sum.0;
    }
    (knot_index, basis)
}