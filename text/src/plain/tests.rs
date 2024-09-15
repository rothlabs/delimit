use futures::executor::block_on;

use super::*;

fn new_list(ace: &Leaf<String>) -> Result<Node<List>> {
    List::new().separator(", ").push("str").push(ace).node()
}

#[test]
fn read_from_list() -> Result<()> {
    let string_leaf = "ace".leaf();
    let text_node = new_list(&string_leaf)?.hub()?;
    block_on(text_node.read(|string| assert_eq!(string, "str, ace")))
}

#[test]
fn solve_same_hub_twice() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    assert!(block_on(text.solve())? == block_on(text.solve())?);
    Ok(())
}

#[test]
fn rebut_from_self() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    let a = block_on(text.solve())?;
    text.write(|pack| {
        pack.unit.set_separator(" > ");
    })?;
    let b = block_on(text.solve())?;
    assert!(a != b);
    Ok(())
}

#[test]
fn react_from_stem() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    let a = block_on(text.solve())?;
    ace.write(|str| str.push_str("_mutated"))?;
    let b = block_on(text.solve())?;
    assert!(a != b);
    Ok(())
}

#[test]
fn no_rebut_after_dropping_stem() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace)?;
    let _r = text.solve();
    text.write(|pack| {
        pack.unit.remove(1);
    })?;
    let a = block_on(text.solve())?;
    ace.write(|str| str.push_str("_mutated"))?;
    let b = block_on(text.solve())?;
    assert!(a == b);
    Ok(())
}
