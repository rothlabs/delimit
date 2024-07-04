// pub trait ToViewsBuilder<L: Reader, E> {
//     fn builder(&mut self) -> ViewsBuilder<L, E>;
// }

// impl<L: Reader, E> ToViewsBuilder<L, E> for Vec<View<L, E>> {
//     fn builder(&mut self) -> ViewsBuilder<L, E> {
//         ViewsBuilder { views: self, reactor: None }
//     }
// }

// pub struct ViewsBuilder<'a, L: Reader, E> {
//     views: &'a mut Vec<View<L, E>>,
//     reactor: Option<&'a Reactor>, 
// }

// impl<'a, L: Reader, E> ViewsBuilder<'a, L, E> {
    
// }