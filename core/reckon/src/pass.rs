use super::*;

impl<'a, G, S> Pass<'a, G, S> {
    fn solve<T>(&'a mut self, hub: &'a Hub<T>) -> Result<&T> 
    where 
        G: Solve<T>,
        S: Store<T>,
    {
        Ok(match hub {
            Hub::Base(base) => base,
            Hub::Apex(key) => {
                if !self.state.has(key) {
                    let base = self.graph.solve(key)?;
                    self.state.set(key, base);
                }
                self.state.get(key)
            },
        })
    }
}

// if !self.state.has(key) {
//     let base = self.graph.solve(key)?;
//     self.state.set(key, base);
// }
// Ok(self.state.get(key))

// if let Some(base) = self.state.get(key) {
//     return Ok(base);
// }
// let base = self.graph.solve(key)?;
// Ok(self.state.set(key, base))

// if let Some(base) = self.state.get(key.clone()) {
//     return Ok(base);
// }
// let base = self.graph.solve(key)?;
// self.state.set(key.clone(), base)

// if let Some(base) = self.state.get(key) {
//     base
// } else {
//     let base = self.graph.solve(key)?;
//     self.state.set(key, base)
// }

// let base = self.graph.solve(key)?;
//                 self.state.set(key, base);
//                 Ok(self.state.get(key)) // .expect("base must exist")
