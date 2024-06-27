use super::*;

#[test]
fn read_leaf_string() {
    let string = "I'm a string leaf!";
    let leaf = string.leaf();
    leaf.reader(|unit| {
        assert_eq!(unit, string);
    });
}
