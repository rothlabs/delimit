use super::*;

fn new_list(ace: &Ace<Load>) -> Agent<List> {
    List::new().separator(", ").push("str").push(ace).link()
}

#[test]
fn grant_and_read_ace_from_list()  { 
    let ace = "ace".ace();
    let text = new_list(&ace);
    text.solve().read_string(|string| {
        assert_eq!(string, "str, ace");
    });
}

#[test]
fn grant_same_link_twice() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    assert!(text.solve() == text.solve());
}

#[test]
fn rebut_from_self() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.solve();
    text.write(|pack| {
        pack.unit.separator(" > ");
    })
    .ok();
    let b = text.solve();
    assert!(a != b);
}

#[test]
fn react_from_stem() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let a = text.solve();
    ace.write(|load| 
        if let Load::String(string) = load {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    ).ok();
    let b = text.solve();
    assert!(a != b);
}

#[test]
fn no_rebut_after_dropping_stem() {
    let ace = "ace".ace();
    let text = new_list(&ace);
    let _r = text.solve();
    text.write(|pack| {
        pack.unit.remove(1);
    })
    .ok();
    let a = text.solve();
    ace.write(|load| 
        if let Load::String(string) = load {
            string.push_str("_mutated");
        } else {
            panic!("was not a string")
        }
    ).ok();
    let b = text.solve();
    assert!(a == b);
}

// ace.write(|string| string.push_str("_mutated")).ok();
