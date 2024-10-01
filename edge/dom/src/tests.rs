use super::*;

#[test]
fn make_buffer() -> Result<()> {
    let canvas = Window::new()?.document()?.body()?.stem("canvas")?;
    Ok(())
}