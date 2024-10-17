use super::*;

#[allow(dead_code)]
pub struct Grid {
    shapes: Vec<Shape>,
}

impl Solve for Grid {
    type Base = Shape;
    async fn solve(&self) -> graph::Result<Hub<Shape>> {
        // let mut shapes = ShapeBuilder::default();
        // for shape in self.shapes {

        // }
        let shape = ShapeBuilder::default().build().unwrap();//.node()?;
        let hub = Hub::Tray(Tray::Base(shape));
        // let test = hub.base().await?;
        Ok(hub)
    }
}

// Ok(Hub::Leaf(Leaf::new(shape)))



// pub struct Grid {
//     shapes: Vec<Node<Shape>>,
// }

// impl Solve for Grid {
//     type Base = Node<Shape>;
//     async fn solve(&self) -> graph::Result<Hub<Node<Shape>>> {
//         // let mut shapes = ShapeBuilder::default();
//         // for shape in self.shapes {

//         // }
//         let shape = ShapeBuilder::default().build().unwrap().node()?;
//         let hub = Hub::Tray(Tray::Base(shape));
//         // let test = hub.base().await?;
//         Ok(hub)
//     }
// }

// // Ok(Hub::Leaf(Leaf::new(shape)))