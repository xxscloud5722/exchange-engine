use std::time;
use std::cmp::Ordering;
use crate::engine::domain::OrderSide;
use std::collections::{BinaryHeap, HashMap};

/// 订单的索引
#[derive(Clone)]
struct OrderIndex {
    id: u64,
    price: f64,
    timestamp: time::SystemTime,
    order_side: OrderSide,
}

impl Ord for OrderIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.price < other.price {
            match self.order_side {
                OrderSide::Sell => Ordering::Less,
                OrderSide::Buy => Ordering::Greater,
            }
        } else if self.price > other.price {
            match self.order_side {
                OrderSide::Buy => Ordering::Greater,
                OrderSide::Sell => Ordering::Less,
            }
        } else {
            // FIFO 如果价格一样根据时间判断
            other.timestamp.cmp(&self.timestamp)
        }
    }
}

impl PartialOrd for OrderIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderIndex {
    fn eq(&self, other: &Self) -> bool {
        if self.price > other.price || self.price < other.price {
            false
        } else {
            self.timestamp == other.timestamp
        }
    }
}

impl Eq for OrderIndex {}


/// 订单队列
pub struct OrderQueue<T> {
    // use Option in order to replace heap in mutable borrow
    idx_queue: Option<BinaryHeap<OrderIndex>>,
    orders: HashMap<u64, T>,
    op_counter: u64,
    max_stalled: u64,
    queue_side: OrderSide,
}

impl<T> OrderQueue<T> {
    /// 创建队列
    pub fn new(side: OrderSide, max_stalled: u64, capacity: usize) -> Self {
        OrderQueue {
            idx_queue: Some(BinaryHeap::with_capacity(capacity)),
            orders: HashMap::with_capacity(capacity),
            op_counter: 0,
            max_stalled,
            queue_side: side,
        }
    }


    /// 将首个元素从队列中弹出，如果队列是空的，就返回 None
    pub fn peek(&mut self) -> Option<&T> {
        // 获取当前订单
        let order_id = self.get_current_order_id()?;

        // 判断订单是否存在
        if self.orders.contains_key(&order_id) {
            self.orders.get(&order_id)
        } else {
            self.idx_queue.as_mut().unwrap().pop()?;
            self.peek()
        }
    }


    /// 删除首元素
    pub fn pop(&mut self) -> Option<T> {
        // 任何情况下都从队列删除订单
        let order_id = self.idx_queue.as_mut()?.pop()?.id;
        // 是否存在否则递归删除
        if self.orders.contains_key(&order_id) {
            self.orders.remove(&order_id)
        } else {
            self.pop()
        }
    }


    // Add new limit order to the queue
    pub fn insert(&mut self, id: u64, price: f64, ts: time::SystemTime, order: T) -> bool {
        if self.orders.contains_key(&id) {
            // do not update existing order
            return false;
        }

        // store new order
        self.idx_queue.as_mut().unwrap().push(OrderIndex {
            id,
            price,
            timestamp: ts,
            order_side: self.queue_side,
        });
        self.orders.insert(id, order);
        true
    }


    // use it when price was changed
    pub fn amend(&mut self, id: u64, price: f64, ts: time::SystemTime, order: T) -> bool {
        if self.orders.contains_key(&id) {
            // store new order data
            self.orders.insert(id, order);
            self.rebuild_idx(id, price, ts);
            true
        } else {
            false
        }
    }


    pub fn cancel(&mut self, id: u64) -> bool {
        match self.orders.remove(&id) {
            Some(_) => {
                self.clean_check();
                true
            }
            None => false,
        }
    }


    /* Internal methods */


    /// Used internally when current order is partially matched.
    ///
    /// Note: do not modify price or time, cause index doesn't change!
    pub fn modify_current_order(&mut self, new_order: T) -> bool {
        if let Some(order_id) = self.get_current_order_id() {
            if self.orders.contains_key(&order_id) {
                self.orders.insert(order_id, new_order);
                return true;
            }
        }
        false
    }


    /// Verify if queue should be cleaned
    fn clean_check(&mut self) {
        if self.op_counter > self.max_stalled {
            self.op_counter = 0;
            self.remove_stalled()
        } else {
            self.op_counter += 1;
        }
    }


    /// Remove dangling indices without orders from queue
    fn remove_stalled(&mut self) {
        if let Some(idx_queue) = self.idx_queue.take() {
            let mut active_orders = idx_queue.into_vec();
            active_orders.retain(|order_ptr| self.orders.contains_key(&order_ptr.id));
            self.idx_queue = Some(BinaryHeap::from(active_orders));
        }
    }


    /// Recreate order-index queue with changed index info
    fn rebuild_idx(&mut self, id: u64, price: f64, ts: time::SystemTime) {
        if let Some(idx_queue) = self.idx_queue.take() {
            // deconstruct queue
            let mut active_orders = idx_queue.into_vec();
            // remove old idx value
            active_orders.retain(|order_ptr| order_ptr.id != id);
            // insert new one
            active_orders.push(OrderIndex {
                id,
                price,
                timestamp: ts,
                order_side: self.queue_side,
            });
            // construct new queue
            let mut amended_queue = BinaryHeap::from(active_orders);
            self.idx_queue = Some(amended_queue);
        }
    }


    /// Return ID of current order in queue
    fn get_current_order_id(&self) -> Option<u64> {
        let order_id = self.idx_queue.as_ref()?.peek()?;
        Some(order_id.id)
    }
}
