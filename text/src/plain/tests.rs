use std::ops::DerefMut;

use super::*;

fn new_list(ace: &Ace<Load>) -> Deuce<List> {
    List::new().separator(", ").push("str").push(ace).link()
    //list.link()
    // let list = ", ".list();
    // list.link
    //     .write(|pack| {
    //         pack.unit.items.back(pack.back).str("str").base(ace);
    //     })
    //     .ok();
    // list.link
}

#[test]
fn grant_and_read_ace_from_list() -> Result<(), String> {
    let ace = "ace".ace();
    let text = new_list(&ace);
    text.grant().read_string(|string| {
        assert_eq!(string, "str, ace");
    })?;
    Ok(())
}

#[test]
fn grant_same_link_twice() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    assert!(text.grant() == text.grant());
}

#[test]
fn rebut_from_self() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.grant();
    text.write(|pack| {
        pack.unit.separator(" > ");
    })
    .ok();
    let b = text.grant();
    assert!(a != b);
}

#[test]
fn react_from_stem() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.grant();
    ace.write(|load| 
        if let Load::String(string) = load {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    ).ok();
    let b = text.grant();
    assert!(a != b);
}

#[test]
fn no_rebut_after_dropping_stem() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let _r = text.grant();
    text.write(|pack| {
        pack.unit.remove(1);
    })
    .ok();
    let a = text.grant();
    ace.write(|load| 
        if let Load::String(string) = load {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    ).ok();
    let b = text.grant();
    assert!(a == b);
}

// ace.write(|string| string.push_str("_mutated")).ok();
