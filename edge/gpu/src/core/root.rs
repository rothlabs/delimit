use super::*;

mod image;

pub struct Image<'a> {
    pub core: &'a Core,
    pub stems: Vec<Hub<Action>>,
}

// pub struct ImageWithPipeline

// pipe: &'a RenderPipeline