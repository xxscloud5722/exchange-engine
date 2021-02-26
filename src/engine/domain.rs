use std::fmt::Debug;

/// 订单类型
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit,
}

/// 订单方向
#[derive(Debug, Copy, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

pub trait Asset {}

//
// /// 订单
// #[derive(Debug, Clone)]
// pub struct Order<Asset> where Asset: Debug + Clone {
//     pub order_id: u64,
//     pub order_asset: Asset,
//     pub price_asset: Asset,
//     pub side: OrderSide,
//     pub price: f64,
//     pub quantity: f64,
// }