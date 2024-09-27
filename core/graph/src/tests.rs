use super::*;

const TEXT: &str = "Turtles are precious.";

#[test]
fn read_leaf() -> Result<()> {
    let leaf = TEXT.leaf();
    leaf.read(|text| assert_eq!(text, TEXT))?;
    Ok(())
}
