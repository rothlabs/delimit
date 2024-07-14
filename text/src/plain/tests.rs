use super::*;

fn new_list(leaf: &Sole<String>) -> Text<List> {
    let list = ", ".list();
    list.link.writer(|pack| {
        pack.unit
            .items
            .back(pack.back)
            .add_str("str")
            .add_sole(leaf);
    });
    list.link
}

#[test]
fn grant_and_read_sole_from_list() {
    let leaf = "leaf".sole();
    let text = new_list(&leaf);
    text.grant().reader(|string| {
        assert_eq!(string, "str, leaf");
    });
}

#[test]
fn grant_same_link_twice() {
    let leaf = "leaf".sole();
    let text = new_list(&leaf);
    assert!(text.grant() == text.grant());
}

#[test]
fn react_from_self() {
    let leaf = "leaf".sole();
    let text = new_list(&leaf);
    let a = text.grant();
    text.writer(|pack| {
        pack.unit.separator(" > ");
    });
    let b = text.grant();
    assert!(a != b);
}

#[test]
fn react_from_stem() {
    let leaf = "leaf".sole();
    let text = new_list(&leaf);
    let a = text.grant();
    leaf.writer(|string| string.push_str("_mutated"));
    let b = text.grant();
    assert!(a != b);
}

#[test]
fn no_reactions_after_dropping_stem() {
    let leaf = "leaf".sole();
    let text = new_list(&leaf);
    let _r = text.grant();
    text.writer(|pack| pack.unit.remove(1));
    let a = text.grant();
    leaf.writer(|string| string.push_str("_mutated"));
    let b = text.grant();
    assert!(a == b);
}
