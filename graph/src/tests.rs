use super::*;

const TEXT: &str = "Turtles are precious.";

fn check_text(tray: &Tray) -> Result<()> {
    match tray {
        Tray::String(value) => Ok(assert_eq!(value, TEXT)),
        tray => Err(tray.wrong_variant("String"))?,
    }
}

#[test]
fn read_leaf() -> Result<()> {
    let leaf = TEXT.leaf();
    leaf.read(check_text)??;
    Ok(())
}
