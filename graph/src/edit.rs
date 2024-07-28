use super::*;

pub struct Field<T> {
    name: String,
    link: T, 
}

pub enum FieldType {
    String(Field<Value<String>>),
    // could have special name for Asset<Deuce<T>> because it will be so common. Maybe thats 
    // what asset should be
    // TextList(Field<Asset<Deuce<List>>>),
}