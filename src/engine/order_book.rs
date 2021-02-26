use std::fmt::Debug;
use crate::engine::order_queues::OrderQueue;
use crate::engine::domain::OrderSide;

const MIN_SEQUENCE_ID: u64 = 1;
const MAX_SEQUENCE_ID: u64 = 1000;
const MAX_STALLED_INDICES_IN_QUEUE: u64 = 10;
const ORDER_QUEUE_INIT_CAPACITY: usize = 500;

/// 订单
#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: u64,
    pub order_asset: String,
    pub price_asset: String,
    pub side: OrderSide,
    pub price: f64,
    pub number: f64,
}


/// 订单账本
pub struct OrderBook {
    order_asset: String,
    price_asset: String,
    buy_queue: OrderQueue<Order>,
    sell_queue: OrderQueue<Order>,
}

impl OrderBook {
    /// 为资产创建订单薄
    pub fn new(order_asset: String, price_asset: String) -> Self {
        OrderBook {
            order_asset,
            price_asset,
            buy_queue: OrderQueue::new(OrderSide::Buy, MAX_STALLED_INDICES_IN_QUEUE, ORDER_QUEUE_INIT_CAPACITY),
            sell_queue: OrderQueue::new(OrderSide::Sell, MAX_STALLED_INDICES_IN_QUEUE, ORDER_QUEUE_INIT_CAPACITY),
        }
    }


    pub fn release_order(&mut self) {}

    pub fn cancel_order(&mut self) {}
}