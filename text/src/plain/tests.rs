use super::*;

fn new_list(ace: &Leaf<String>) -> Result<Node<List>> {
    List::new().separator(", ").push("str").push(ace).node()
}

#[tokio::test]
async fn read_from_list() -> Result<()> {
    let string_leaf = "ace".leaf();
    let text_node = new_list(&string_leaf)?.hub()?;
    text_node.read(|string| assert_eq!(string, "str, ace")).await
}

#[tokio::test]
async fn solve_same_hub_twice() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    assert!(text.solve().await? == text.solve().await?);
    Ok(())
}

#[tokio::test]
async fn rebut_from_self() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    let a = text.solve().await?;
    text.write(|pack| {
        pack.unit.set_separator(" > ");
    })?;
    let b = text.solve().await?;
    assert!(a != b);
    Ok(())
}

#[tokio::test]
async fn react_from_stem() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    let a = text.solve().await?;
    ace.write(|str| str.push_str("_mutated")).await?;
    let b = text.solve().await?;
    assert!(a != b);
    Ok(())
}

#[tokio::test]
async fn no_rebut_after_dropping_stem() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    let _r = text.solve();
    text.write(|pack| {
        pack.unit.remove(1);
    })?;
    let a = text.solve().await?;
    ace.write(|str| str.push_str("_mutated")).await?;
    let b = text.solve().await?;
    assert!(a == b);
    Ok(())
}
