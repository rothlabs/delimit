use super::*;

fn new_list(ace: &Leaf) -> Node<List> {
    List::new().separator(", ").push("str").push(ace).node()
}

#[test]
fn read_from_list() -> Result<(), Error> {
    let ace = "ace".leaf();
    let text = new_list(&ace).apex();
    text.view().string(|string| 
        Ok(assert_eq!(string?, "str, ace"))
    )//;
    // Ok(())
}

#[test]
fn solve_same_apex_twice() -> Result<(), Error> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    assert!(text.main()? == text.main()?);
    Ok(())
}

#[test]
fn rebut_from_self() -> Result<(), Error> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    let a = text.solve(Task::Main)?;
    text.write(|pack| {
        pack.unit.set_separator(" > ");
        Ok(())
    })?;
    let b = text.solve(Task::Main)?;
    assert!(a != b);
    Ok(())
}

#[test]
fn react_from_stem() -> Result<(), Error> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    let a = text.solve(Task::Main)?;
    ace.write(|tray| {
        if let Tray::String(string) = tray {
            string.push_str("_mutated");
            Ok(())
        } else {
            panic!("was not a string")
        }
    })
    .ok();
    let b = text.solve(Task::Main)?;
    assert!(a != b);
    Ok(())
}

#[test]
fn no_rebut_after_dropping_stem() -> Result<(), Error> {
    let ace = "ace".leaf();
    let text = new_list(&ace);
    let _r = text.solve(Task::Main);
    text.write(|pack| {
        pack.unit.remove(1);
        Ok(())
    })
    .ok();
    let a = text.solve(Task::Main)?;
    ace.write(|tray| {
        if let Tray::String(string) = tray {
            string.push_str("_mutated");
            Ok(())
        } else {
            panic!("was not a string")
        }
    })
    .ok();
    let b = text.solve(Task::Main)?;
    assert!(a == b);
    Ok(())
}

// ace.write(|string| string.push_str("_mutated")).ok();
