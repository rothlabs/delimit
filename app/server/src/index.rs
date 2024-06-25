use chronicle::text::list;
use graph::{
    link::{Leaf, Write},
    //AddLink,
    FromUnit,
}; // html::doc,

pub fn index() -> String {
    let leaf = Leaf::from_unit("cool".to_owned());
    let mut list = list();
    list.separator(" _sep_ ");
    let mut text = list.text();
    text.add_leaf(&leaf);
    println!("{}", text.string());
    leaf.write(|unit| unit.push_str(" changed!"));
    println!("{}", text.string());
    text.add_leaf(&leaf);
    leaf.write(|unit| unit.push_str(" wow"));
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
