use super::*;

#[test]
fn read_ace_string() {
    let string = "I'm a ace string!";
    let leaf = string.leaf(); //Ace::new(string.to_owned());
    leaf.read(|load| match load {
        Load::String(s) => assert_eq!(s, string),
        _ => panic!("not a string"),
    });
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
