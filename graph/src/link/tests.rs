use super::*;

#[test]
fn read_ace_string() {
    let string = "I'm a ace string!";
    let ace = Ace::new(string.to_owned());
    ace.read(|load| assert_eq!(load, string));
}

// #[test]
// fn read_ace_string() {
//     let string = "I'm a ace string!";
//     let ace = string.ace();
//     ace.read(|load| {
//         if let Load::String(s) = load {
//             assert_eq!(s, string);
//         } else {
//             panic!("not a string")
//         }
//     });
// }
