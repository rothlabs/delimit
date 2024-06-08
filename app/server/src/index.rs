use std::borrow::BorrowMut;

use chronicle::{html::doc, text::{list, node::{leaf, Leaf, List, Node}}};

pub fn index() -> String {
    //let mut su = string_unit("cool");
    let mut leaf = leaf("cool");
    let mut list = list();
    list.add_leaf(leaf.clone());
    //let wow = leaf.string.borrow_mut();

    let mut html = doc().html();
    html.lang("en");
    let mut title = html.head().title();
    title.leaf("Delimit");
    let mut meta = title.root().meta();
    meta.charset("utf-8");
    meta = meta.root().meta();
    meta.name("viewport")
        .content("width=device-width, initial-scale=1");
    meta = meta.root().meta();
    meta.name("author").content("Roth Labs LLC");
    let doc = meta.up_to_doc();
    let text = doc.text();

    // let wow = text.any().downcast_ref::<Leaf>().unwrap();
    // // let wo2 = wow.items.last().unwrap().any().downcast_ref::<Leaf>().unwrap();
    // // let wo3 = wo2.string().0;
    // // println!("wo3 = {}", wo3);
    let json = text.serialize();
    println!("{json}");
    doc.string()
}
