use serialize_macro::{SerializeStruct,DeserializeStruct};
use serialize_macro_traits::{Serialize,Deserialize};
use std::fmt::Error;

#[derive(SerializeStruct,DeserializeStruct)]
#[derive(Debug)]
struct Swap{
  name:String,
  age:u8,
  salary:u32
}

pub fn main(){
  let s=Swap{
    name:String::from("adil shaikh"),
    age:23,
    salary:1234
  };

  let bytes=s.serialize();
  println!("{:?}",bytes);

  let sn=Swap::deserialize(&bytes).expect("failed to deserialize");
  println!("{:?}",sn);
}