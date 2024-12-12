use super::*;

pub mod compute;
pub mod render;
pub mod command;

#[derive(Debug)]
pub enum Command {
    Compute(compute::Pass),
    Render(render::Pass),
}