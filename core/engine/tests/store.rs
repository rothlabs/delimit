use engine::*;

#[test]
fn gen_vec() {
    let mut gv = Store::default();

    // Insert
    let a = gv.insert("a");
    let b = gv.insert("b");
    let c = gv.insert("c");
    assert_eq!(gv.get(&a), Some(&"a"));
    assert_eq!(gv.get(&b), Some(&"b"));
    assert_eq!(gv.get(&c), Some(&"c"));
    assert_eq!(gv.len, 3);

    // Remove
    gv.remove(&a);
    assert_eq!(gv.get(&a), None);
    assert_eq!(gv.len, 2);

    // Re-insert
    let d = gv.insert("d");

    assert_eq!(a.index, d.index);
    assert_ne!(a.generation, d.generation);

    // Re-remove and re-re-insert
    gv.remove(&d);
    let e = gv.insert("e");
    assert_eq!(a.index, e.index);
    assert_ne!(a.generation, e.generation);
}