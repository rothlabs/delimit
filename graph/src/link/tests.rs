use super::*;

#[test]
fn read_ace_string() {
    let string = "I'm a ace string!";
    let ace = string.ace();
    ace.read(|unit| {
        assert_eq!(unit, string);
    });
}
