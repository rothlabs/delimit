use chronicle::text::list;
use graph::{link::{Leaf, Write}, New}; // html::doc,


pub fn index() -> String {
    let leaf = Leaf::new("cool".to_owned());
    let mut list = list();
    list.add_leaf(&leaf);
    list.add_leaf(&leaf);
    list.add_leaf(&leaf);
    let text = list.text();
    println!("{}", text.string());
    leaf.write(|unit| unit.push_str("test")); // leaf.write(|unit| unit.push_str("test"));
    println!("{}", text.string());

    String::new()
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
