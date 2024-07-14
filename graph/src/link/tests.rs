use super::*;

#[test]
fn read_leaf_string() {
    let string = "I'm a leaf string!";
    let leaf = string.ace();
    leaf.reader(|unit| {
        assert_eq!(unit, string);
    });
}
