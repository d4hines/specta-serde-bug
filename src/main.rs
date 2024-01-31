use specta::{ts, Type};

#[derive(Type)]

pub struct MyStruct {
    foo: String,
    bar: String,
}

#[derive(Type)]
#[serde(tag = "type")]
pub enum MyEnum {
    A(Option<MyStruct>),
    B,
}

fn main() {
    assert_eq!(
        ts::export::<MyEnum>(&Default::default()).unwrap(),
        "export type MyEnum = ({ type: \"A\" } & (MyStruct | null)) | { type: \"B\" }".to_string()
    );
    println!("ok!");
}
