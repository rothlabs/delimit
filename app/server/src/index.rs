//use chronicle::{html::doc, text::{list, node::{leaf, Node}}};

use chronicle::{html::unit::doc, text::unit::list};
use graph::LeafStr;

pub fn index() -> String {
    let leaf = LeafStr::new("cool");
    let mut list = list();
    list.add_leaf(&leaf);
    list.add_leaf(&leaf);
    list.add_leaf(&leaf);
    let text = list.text();
    println!("{}", text.string());
    leaf.write(|unit| unit.push_str("test"));
    //leaf.write().meta.node.id = Id::new();
    println!("{}", text.string());

    let mut html = doc().html();
    html.lang("en");
    let mut title = html.head().title();
    title.add_str("Delimit");
    let mut meta = title.root().meta();
    meta.charset("utf-8");
    meta = meta.root().meta();
    meta.name("viewport")
        .content("width=device-width, initial-scale=1");
    meta = meta.root().meta();
    meta.name("author").content("Roth Labs LLC");
    let mut body = meta.up_to_html().body();
    body.add_str("Let's roll.");
    let text = body.up_to_doc().text();
    //let text = doc.text();

    // let wow = text.any().downcast_ref::<Leaf>().unwrap();
    // // let wo2 = wow.items.last().unwrap().any().downcast_ref::<Leaf>().unwrap();
    // // let wo3 = wo2.string().0;
    // // println!("wo3 = {}", wo3);
    let json = text.serial();
    println!("{json}");
    let t = text.string();
    println!("{t}");
    t
}
