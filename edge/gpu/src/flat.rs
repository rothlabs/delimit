use super::*;

pub mod command;
pub mod compute;
pub mod render;

#[derive(Debug)]
pub enum Command {
    Compute(compute::Pass),
    Render(render::Pass),
}
