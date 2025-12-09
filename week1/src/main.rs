use std::fmt::Error;

#[allow(dead_code)]
trait Serialize{
  fn serialize(&self)->Vec<u8>;
}
#[allow(dead_code)]
trait Deserialize:Sized{
  fn deserialize(base: &[u8])->Result<Self,Error>;
}

#[allow(dead_code)]
#[derive(Debug)]
struct Swap{
  qty_1:u32,
  qty_2:u32
}
impl Serialize for Swap{
  fn serialize(&self)->Vec<u8> {
      let mut res=Vec::new();
      res.extend_from_slice(&self.qty_1.to_be_bytes());
      res.extend_from_slice(&self.qty_2.to_be_bytes());
      res
  }
}
impl Deserialize for Swap{
  fn deserialize(base:&[u8])->Result<Self,Error>{
    if base.len()<8 {
      return Err(Error);
    }
    let b1=base[0..4].try_into().map_err(|_| Error)?;
    let val1=u32::from_be_bytes(b1);
    let b2=base[4..8].try_into().map_err(|_| Error)?;
    let val2=u32::from_be_bytes(b2);

    Ok(Swap{qty_1:val1,qty_2:val2})
  }
}

fn main(){
  let s=Swap{
    qty_1:2343,
    qty_2:45
  };
  let ser=s.serialize();
  println!("{:?}",ser);
  let deser= Swap::deserialize(&ser).unwrap();
  println!("{:?}",deser);
}