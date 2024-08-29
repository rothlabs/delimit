use super::*;

fn new_list(ace: &Leaf) -> Node<List> {
    List::new().separator(", ").push("str").push(ace).node()
}

#[test]
fn read_from_list() -> Result<()> {
    let string_leaf = "ace".leaf();
    let text_node = new_list(&string_leaf).apex();
    text_node
        .view()
        .string(|string| assert_eq!(string, "str, ace"))
}

#[test]
fn solve_same_apex_twice() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    assert!(text.main()? == text.main()?);
    Ok(())
}

#[test]
fn rebut_from_self() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    let a = text.solve(Task::Main)?;
    text.write(|pack| {
        pack.unit.set_separator(" > ");
    })?;
    let b = text.solve(Task::Main)?;
    assert!(a != b);
    Ok(())
}

#[test]
fn react_from_stem() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    let a = text.solve(Task::Main)?;
    ace.write(|tray| {
        if let Tray::String(string) = tray {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    })?;
    let b = text.solve(Task::Main)?;
    assert!(a != b);
    Ok(())
}

#[test]
fn no_rebut_after_dropping_stem() -> Result<()> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    let _r = text.solve(Task::Main);
    text.write(|pack| {
        pack.unit.remove(1);
    })?;
    let a = text.solve(Task::Main)?;
    ace.write(|tray| {
        if let Tray::String(string) = tray {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    })?;
    let b = text.solve(Task::Main)?;
    assert!(a == b);
    Ok(())
}

// ace.write(|string| string.push_str("_mutated")).ok();
