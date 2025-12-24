use serialize_macro::{DeserializeFixedStruct, SerializeFixedStruct};
use serialize_macro_traits::{Serialize,Deserialize};
use std::fmt::Error;


#[derive(SerializeFixedStruct,DeserializeFixedStruct)]
#[derive(Debug)]
struct Swap{
  qty_1:u32,
  qty_2:usize,
  qty_3:i8
}

pub fn main(){
  let s=Swap{
    qty_1:2,
    qty_2:3,
    qty_3:4
  };

  let bytes=s.serialize();
  println!("{:?}",bytes);

  let sn:Swap=Swap::deserialize(&bytes).expect("Failed to deserialize");
  println!("{:?}",sn);
}