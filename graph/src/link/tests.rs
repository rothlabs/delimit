use super::*;

#[test]
fn read_ace_string() {
    let string = "I'm a ace string!";
    let leaf = string.leaf(); //Ace::new(string.to_owned());
    leaf.read(|tray| match tray {
        Tray::String(s) => Ok(assert_eq!(s, string)),
        _ => panic!("not a string"),
    });
}

// #[test]
// fn read_ace_string() {
//     let string = "I'm a ace string!";
//     let ace = string.ace();
//     ace.read(|tray| {
//         if let Tray::String(s) = tray {
//             assert_eq!(s, string);
//         } else {
//             panic!("not a string")
//         }
//     });
// }
