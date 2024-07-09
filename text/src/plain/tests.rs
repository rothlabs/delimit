use super::*;

fn new_list(leaf: &Leaf<String>) -> Text<List> {
    let (_, text_list) = ", ".list();
    text_list.writer(|pack| {
        pack.unit
            .items
            .root(pack.root)
            .add_str("str")
            .add_leaf(leaf);
    });
    text_list
}

#[test]
fn solve_list_as_string() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    text.solve().reader(|string| {
        assert_eq!(string, "str, leaf");
    });
}

#[test]
fn solve_list_as_leaf() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    text.solve().reader(|string| {
        assert_eq!(string, "str, leaf");
    });
}

#[test]
fn solve_twice_for_same_link() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    assert!(text.solve() == text.solve());
}

#[test]
fn react_from_self() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    let a = text.solve();
    text.writer(|pack| {
        pack.unit.separator(" > ");
    });
    let b = text.solve();
    assert!(a != b);
}

#[test]
fn react_from_stem() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    let a = text.solve();
    leaf.writer(|string| string.push_str("_mutated"));
    let b = text.solve();
    assert!(a != b);
}

#[test]
fn no_reactions_after_dropping_stem() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    let _r = text.solve();
    text.writer(|pack| pack.unit.remove(1));
    let a = text.solve();
    leaf.writer(|string| string.push_str("_mutated"));
    let b = text.solve();
    assert!(a == b);
}
