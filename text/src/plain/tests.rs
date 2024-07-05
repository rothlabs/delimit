use super::*;

fn new_list(leaf: &Leaf<String>) -> crate::plain::Role {
    let text = ", ".list();
    text.list().writer_pack(|pack| {
        pack.unit.items.add_str("str").add_leaf(leaf, pack.reactor);
    });
    text
}

#[test]
fn solve_list_as_string() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    text.list().solve().reader(|string| {
        assert_eq!(string, "str, leaf");
    });
}

#[test]
fn solve_list_as_leaf() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    text.list().solve().reader(|string| {
        assert_eq!(string, "str, leaf");
    });
}

#[test]
fn solve_twice_for_same_link() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    assert!(text.list().solve() == text.list().solve());
}

#[test]
fn react_from_self() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    let a = text.list().solve();
    text.list().writer(|list| {
        list.separator(" > ");
    });
    let b = text.list().solve();
    assert!(a != b);
}

#[test]
fn react_from_stem() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    let a = text.list().solve();
    leaf.writer(|string| string.push_str("_mutated"));
    let b = text.list().solve();
    assert!(a != b);
}

#[test]
fn no_reactions_after_dropping_stem() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    let _r = text.list().solve();
    text.list().writer(|list| list.remove(1));
    let a = text.list().solve();
    leaf.writer(|string| string.push_str("_mutated"));
    let b = text.list().solve();
    assert!(a == b);
}
