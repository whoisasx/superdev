use::std::fmt::Error;
use serialize_macro::{SerializeNumberStruct,DeserializeNumberStruct};
use serialize_macro_traits::{Serialize,Deserialize};


#[derive(SerializeNumberStruct,DeserializeNumberStruct)]
struct Swap{
  val_1:i32,
  val_2:i32,
  val_3:i32
}

fn main(){
  println!("hello world");
  let s=Swap{
    val_1:1,
    val_2:2,
    val_3:1000
  };

  let bytes=s.serialize();
  println!("{:?}", bytes);
}