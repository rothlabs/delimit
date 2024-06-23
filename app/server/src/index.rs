use chronicle::text::list;
use graph::{
    //AddLink,
    FromUnit,
    link::{Leaf, Write},
}; // html::doc,

pub fn index() -> String {
    let leaf = Leaf::new("cool".to_owned());
    let mut list = list();
    // list.add_leaf(&leaf);
    list.add_str(" wow ");
    // list.add_leaf(&leaf);
    let text = list.text();
    text.ad
    println!("{}", text.string());
    leaf.write(|unit| unit.push_str(" changed!"));
    println!("{}", text.string());

    text.string()
    // let mut html = doc().html();
    // html.lang("en");
    // let mut title = html.head().title();
    // title.add_str("Delimit");
    // let mut meta = title.root().meta();
    // meta.charset("utf-8");
    // meta = meta.root().meta();
    // meta.name("viewport")
    //     .content("width=device-width, initial-scale=1");
    // meta = meta.root().meta();
    // meta.name("author").content("Roth Labs LLC");
    // let mut body = meta.up_to_html().body();
    // body.add_str("Let's roll.");
    // let text = body.up_to_doc().text();
    // let json = text.serial();
    // println!("{json}");
    // let t = text.string();
    // println!("{t}");
    // t
}
