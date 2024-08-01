use super::*;

fn new_list(ace: &Leaf) -> Agent<List> {
    List::new().separator(", ").push("str").push(ace).agent()
}

#[test]
fn read_from_list() -> Result<(), Error> {
    let ace = "ace".ace();
    let text = new_list(&ace);
    text.node().read_string(|string| {
        assert_eq!(string, "str, ace");
    });
    Ok(())
}

#[test]
fn solve_same_node_twice() -> Result<(), Error> {
    let ace = "ace".ace();
    let text = new_list(&ace);
    assert!(text.solve()? == text.solve()?);
    Ok(())
}

#[test]
fn rebut_from_self() -> Result<(), Error> {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.solve()?;
    text.write(|pack| {
        pack.unit.separator(" > ");
    })
    .ok();
    let b = text.solve()?;
    assert!(a != b);
    Ok(())
}

#[test]
fn react_from_stem() -> Result<(), Error> {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.solve()?;
    ace.write(|load| {
        if let Load::String(string) = load {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    })
    .ok();
    let b = text.solve()?;
    assert!(a != b);
    Ok(())
}

#[test]
fn no_rebut_after_dropping_stem() -> Result<(), Error> {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let _r = text.solve();
    text.write(|pack| {
        pack.unit.remove(1);
    })
    .ok();
    let a = text.solve()?;
    ace.write(|load| {
        if let Load::String(string) = load {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    })
    .ok();
    let b = text.solve()?;
    assert!(a == b);
    Ok(())
}

// ace.write(|string| string.push_str("_mutated")).ok();
