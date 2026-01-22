use std::{collections::HashMap};
use crate::{input::Side, output::{DeleteOrderResponse, Depth}};

pub struct Orderbook{
  pub bids: HashMap<u32, Vec<UserOrder>>,
  pub asks: HashMap<u32, Vec<UserOrder>>,
  pub order_id_idx: u32
}

pub struct UserOrder{
  pub user_id: u32,
  pub qty: u32,
  pub fulfilled: u32,
  pub order_id: u32
}

impl Orderbook{
  pub fn new()->Self{
    Self { 
      bids: HashMap::new(), 
      asks: HashMap::new(),
      order_id_idx:0
    }
  }

  pub fn create_order(&mut self, price: u32, quantity: u32, user_id: u32, side: Side)->u32{
    let order_id=self.order_id_idx;
    self.order_id_idx+=1;

    match side{
      Side::Buy =>{
        self.bids.entry(price).or_insert(Vec::new()).push(UserOrder{
          user_id,
          qty:quantity,
          fulfilled:0,
          order_id
        })
      },
      Side::Sell =>{
        self.asks.entry(price).or_insert(Vec::new()).push(UserOrder{
          user_id,
          qty:quantity,
          fulfilled:0,
          order_id,
        })
      }
    }

    order_id
  }

  pub fn delete_order(&mut self, order_id: u32) -> DeleteOrderResponse{
    print!("{}", order_id);

    let mut deleted_order=DeleteOrderResponse{
      filled_qty: 0,
      average_price: 0
    };

    for (price,orders) in &mut self.bids{
      orders.retain(|order| {
        if order_id==order.order_id {
          deleted_order.filled_qty=order.fulfilled;
          deleted_order.average_price=*price;
          return false;
        }
        return true;
      });
    }

    deleted_order
  }

  pub fn get_orders(&self)->Depth{
    let mut recent_bids: Vec<[u32; 2]> = Vec::new();
    let mut recent_asks: Vec<[u32; 2]> = Vec::new();

    let mut keys: Vec<&u32>=self.bids.keys().collect();
    keys.sort();

    let mut cnt=0;
    for price in keys.into_iter().rev() {
      if let Some(orders)= self.bids.get(price) {
        let mut total_qty=0;
        for order in orders {
          total_qty+=order.qty;
        }
        recent_bids.push([*price,total_qty]);
        cnt+=1;
        if cnt==50 {
          break;
        }
      }
    }

    keys=self.asks.keys().collect();
    keys.sort();
    cnt=0;
    for price in keys.into_iter() {
      if let Some(orders)=self.bids.get(price) {
        let mut total_qty=0;
        for order in orders {
          total_qty+=order.qty;
        }

        recent_asks.push([*price,total_qty]);
        cnt+=1;
        if cnt==50 {break;}
      }
    }

    Depth { 
      bids: recent_bids, 
      asks: recent_asks, 
      last_updated_id: 1 
    }
  }
}