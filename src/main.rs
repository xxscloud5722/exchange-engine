use crate::engine::order_book::OrderBook;

mod engine;

fn main() {
    println!("Hello, world!");

    // 根据配置文件监听Redis 队列
    let mut order_book = OrderBook::new(String::from("BTC"), String::from("USDT"));
    order_book.release_order();
    order_book.cancel_order();
}
