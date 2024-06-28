use graph::*;

use super::*;

fn new_list(leaf: &Leaf<String>) -> Text<List> {
    let text = ", ".text_list();
    text.writer(|list| {
        list.add_str("str");
    });
    text.stemmer(leaf, |list, leaf| {
        list.add_leaf(leaf);
    });
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
    // text.writer(|list| {
    //     list.separator(" wow ");
    // });
    let result1 = text.leaf();
    result0.reader(|unit| {
        println!("result0 {}", unit);
    });
    result1.reader(|unit| {
        println!("result1 {}", unit);
    });
    assert!(result0 == result1);
}

// #[test]
// fn react_list() {
//     let leaf = "leaf".leaf();
//     let text = new_text_list(&leaf);
//     leaf.write(|unit| unit.push_str("_mutated"));
//     assert_eq!(text.string(), "str, leaf_mutated");
// }
