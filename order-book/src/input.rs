use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Clone,Copy,Debug)]
pub struct CreateOrderInput{
  pub quantity: u32,
  pub price: u32,
  pub user_id: u32,
  pub side: Side
}

#[derive(Deserialize,Clone, Copy,Serialize, Debug)]
pub enum Side{
  Buy,
  Sell
}

#[derive(Deserialize,Serialize)]
pub struct DeleteOrder{
  pub order_id: String
}