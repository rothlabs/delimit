use super::*;

fn new_list(ace: &Ace<String>) -> Link<List> {
    let list = ", ".list();
    list.link.write(|pack| {
        pack.unit.items.back(pack.back).str("str").base(ace);
    });
    list.link
}

#[test]
fn grant_and_read_ace_from_list() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    text.grant().read(|string| {
        assert_eq!(string, "str, ace");
    });
}

#[test]
fn grant_same_link_twice() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    assert!(text.grant() == text.grant());
}

#[test]
fn react_from_self() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.grant();
    text.write(|pack| {
        pack.unit.separator(" > ");
    });
    let b = text.grant();
    assert!(a != b);
}

#[test]
fn react_from_stem() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.grant();
    ace.write(|string| string.push_str("_mutated"));
    let b = text.grant();
    assert!(a != b);
}

#[test]
fn no_reactions_after_dropping_stem() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let _r = text.grant();
    text.write(|pack| pack.unit.remove(1));
    let a = text.grant();
    ace.write(|string| string.push_str("_mutated"));
    let b = text.grant();
    assert!(a == b);
}
