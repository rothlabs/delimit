use super::*;

const BASIC_STRUCT: &str = r#"pub Leaf<L> {
    load: L,
    reactors: Reactors,
}"#;

#[test]
fn make_struct() {
    let (generics, generics_exact) = generics();
    generics_exact.writer(|pack|{
        pack.unit.name = plain::string("Leaf");
        pack.unit.fields = vec![plain::string("L")];
    });
    let (_, struct_rs) = struct_rs();
    struct_rs.writer(|pack| {
        pack.unit.name = generics.reactor(pack.reactor);
    });
    let wow = struct_rs.solve().solve();
    println!("rust struct");
    println!("{}", BASIC_STRUCT);
    println!("{}", wow.load());
    assert!(BASIC_STRUCT == wow.load());
}
