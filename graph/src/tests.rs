use super::*;

const TEXT: &str = "Turtles are precious.";

fn check_text(tray: crate::Result<&Tray>) -> crate::Result<()> {
    match tray? {
        Tray::String(value) => assert_eq!(value, TEXT),
        tray => Err(tray.wrong_variant("String"))?,
    }
    Ok(())
}

#[test]
fn read_leaf() -> crate::Result<()> {
    let leaf = TEXT.leaf();
    leaf.read(check_text)?;
    Ok(())
}
