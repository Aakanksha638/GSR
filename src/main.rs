use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
struct Order {
    id: u64,
    side: Side,
    price: u64,    // price in integer ticks
    quantity: u64,
}

struct OrderBook {
    bids: BTreeMap<u64, VecDeque<Order>>,
    asks: BTreeMap<u64, VecDeque<Order>>,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        let book = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        book.entry(order.price)
            .or_default()
            .push_back(order);
    }

    fn best_bid(&self) -> Option<u64> {
        self.bids.keys().next_back().copied()
    }

    fn best_ask(&self) -> Option<u64> {
        self.asks.keys().next().copied()
    }
}

fn main() {
    let mut book = OrderBook::new();

    book.add_order(Order {
        id: 1,
        side: Side::Buy,
        price: 100,
        quantity: 5,
    });

    book.add_order(Order {
        id: 2,
        side: Side::Buy,
        price: 101,
        quantity: 3,
    });

    book.add_order(Order {
        id: 3,
        side: Side::Sell,
        price: 103,
        quantity: 2,
    });

    book.add_order(Order {
        id: 4,
        side: Side::Sell,
        price: 102,
        quantity: 4,
    });

    println!("Best bid: {:?}", book.best_bid());
    println!("Best ask: {:?}", book.best_ask());
}