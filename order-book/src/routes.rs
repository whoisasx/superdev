use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, Responder, delete, get, post, web::{Data, Json}};

use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::{ Orderbook}, output::CreateOrderResponse};

#[post("/order")]
pub async fn create_order(body:Json<CreateOrderInput>, orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder{
  let price=body.0.price;
  let quantity=body.0.quantity;
  let user_id= body.0.user_id;
  let side=body.0.side;

  let mut orderbook=orderbook.lock().unwrap();
  let new_order_id=orderbook.create_order(price, quantity, user_id, side);
  
  HttpResponse::Ok().json(CreateOrderResponse{
    order_id: new_order_id.to_string()
  })
}

#[delete("/order")]
pub async fn delete_order(Json(body):Json<DeleteOrder>, orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder{
  let order_id= body.order_id;
  let mut orderbook = orderbook.lock().unwrap();
  let delete_response= orderbook.delete_order(order_id.parse::<u32>().unwrap());

  HttpResponse::Ok().json(delete_response)
}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder{
  let orderbook=orderbook.lock().unwrap();
  let depth_response=orderbook.get_orders();

  HttpResponse::Ok().json(depth_response)
}