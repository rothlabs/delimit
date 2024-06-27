use graph::*;

use super::*;

fn new_list(leaf: &Leaf<String>) -> Text<List> {
    let text = ", ".text_list();
    text.writer(|list| {
        list.add_str("str");
        list.add_leaf(leaf);
    });
    // text.stem(leaf, |list, leaf| {
    //     list.add_leaf(leaf);
    // });
    text
}

#[test]
fn solve_list_as_string() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    assert_eq!(text.string(), "str, leaf");
}

#[test]
fn solve_list_as_leaf() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    text.leaf().reader(|string| {
        assert_eq!(string, "str, leaf");
    });
}

#[test]
fn solve_list_twice() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    assert!(text.leaf() == text.leaf());
}

#[test]
fn react_list() {
    let leaf = "leaf".leaf();
    let text = new_list(&leaf);
    let result0 = text.leaf();
    leaf.writer(|string| string.push_str("_mutated"));
    let result1 = text.leaf();
    assert!(result0 != result1);
}



// #[test]
// fn react_list() {
//     let leaf = "leaf".leaf();
//     let text = new_text_list(&leaf);
//     leaf.write(|unit| unit.push_str("_mutated"));
//     assert_eq!(text.string(), "str, leaf_mutated");
// }
