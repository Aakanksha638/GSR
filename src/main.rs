use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
struct Order {
    id: u64,
    side: Side,
    price: u64,
    quantity: u64,
}

#[derive(Debug)]
struct Trade {
    maker_order_id: u64,
    taker_order_id: u64,
    price: u64,
    quantity: u64,
}

struct OrderBook {
    bids: BTreeMap<u64, VecDeque<Order>>,
    asks: BTreeMap<u64, VecDeque<Order>>,
    order_prices: HashMap<u64, (Side, u64)>,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_prices: HashMap::new(),
        }
    }

    fn cancel_order(&mut self, order_id: u64) -> bool {
        let (side, price) = match self.order_prices.remove(&order_id){
            Some(location) => location,
            None => return false,
        };
        let book = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        if let Some(queue) = book.get_mut(&price) {
            if let Some(index) = queue.iter().position(|order| order.id == order_id) {
                queue.remove(index);

                if queue.is_empty() {
                    book.remove(&price);
                }
                return true;
            }

        }
    false 
}

    fn add_order(&mut self, mut incoming: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        match incoming.side {
            Side::Buy => {
                while incoming.quantity > 0 {
                    let best_ask = match self.asks.keys().next().copied() {
                        Some(price) => price,
                        None => break,
                    };

                    // Buy only matches if bid >= ask.
                    if incoming.price < best_ask {
                        break;
                    }

                    let queue = self.asks.get_mut(&best_ask).unwrap();

                    let maker = queue.front_mut().unwrap();

                    let traded_quantity =
                        incoming.quantity.min(maker.quantity);

                    trades.push(Trade {
                        maker_order_id: maker.id,
                        taker_order_id: incoming.id,
                        price: maker.price,
                        quantity: traded_quantity,
                    });

                    maker.quantity -= traded_quantity;
                    incoming.quantity -= traded_quantity;

                    if maker.quantity == 0 {
                        let filled_id = maker.id; // Remember its ID before removing it
                        self.order_prices.remove(&filled_id); // Remove its index entry
                    }

                    if queue.is_empty() {
                        self.asks.remove(&best_ask);
                    }
                }

                // Remaining quantity becomes a resting bid.
                if incoming.quantity > 0 {
                    self.order_prices.insert(incoming.id, (incoming.side, incoming.price));  //save the orders location
                    self.bids.entry(incoming.price).or_default().push_back(incoming); //Add the order to the bid queue.
                }
            }

            Side::Sell => {
                while incoming.quantity > 0 {
                    let best_bid = match self.bids.keys().next_back().copied() {
                        Some(price) => price,
                        None => break,
                    };

                    // Sell only matches if ask <= bid.
                    if incoming.price > best_bid {
                        break;
                    }

                    let queue = self.bids.get_mut(&best_bid).unwrap();

                    let maker = queue.front_mut().unwrap();

                    let traded_quantity =
                        incoming.quantity.min(maker.quantity);

                    trades.push(Trade {
                        maker_order_id: maker.id,
                        taker_order_id: incoming.id,
                        price: maker.price,
                        quantity: traded_quantity,
                    });

                    maker.quantity -= traded_quantity;
                    incoming.quantity -= traded_quantity;

                    if maker.quantity == 0 {
                        queue.pop_front();
                    }

                    if queue.is_empty() {
                        self.bids.remove(&best_bid);
                    }
                }

                // Remaining quantity becomes a resting ask.
                if incoming.quantity > 0 {
                    self.asks
                        .entry(incoming.price)
                        .or_default()
                        .push_back(incoming);
                }
            }
        }

        trades
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

    // Existing sell order:
    book.add_order(Order {
        id: 1,
        side: Side::Sell,
        price: 100,
        quantity: 5,
    });

    // Incoming buy crosses the ask.
    let trades = book.add_order(Order {
        id: 2,
        side: Side::Buy,
        price: 101,
        quantity: 3,
    });

    for trade in trades {
        println!(
            "TRADE: maker={}, taker={}, price={}, quantity={}",
            trade.maker_order_id,
            trade.taker_order_id,
            trade.price,
            trade.quantity
        );
    }

    println!("Best bid: {:?}", book.best_bid());
    println!("Best ask: {:?}", book.best_ask());
    println!("Cancelled: {}", book.cancel_order(1)); // Try cancelling order 1
    println!("Cancelled again: {}", book.cancel_order(1)); // Try cancelling it twice
}

#[cfg(test)] // Run this module only during testing
mod tests {
    use super::*; // Import the order book types and methods

    #[test] // Tell Rust this is a test
    fn finds_best_bid_and_ask() {
        let mut book = OrderBook::new(); // Create an empty order book

        book.add_order(Order { // Add the first buy order
            id: 1,
            side: Side::Buy,
            price: 100,
            quantity: 5,
        });

        book.add_order(Order { // Add a higher-priced buy order
            id: 2,
            side: Side::Buy,
            price: 105,
            quantity: 3,
        });

        book.add_order(Order { // Add the first sell order
            id: 3,
            side: Side::Sell,
            price: 110,
            quantity: 2,
        });

        book.add_order(Order { // Add a lower-priced sell order
            id: 4,
            side: Side::Sell,
            price: 108,
            quantity: 4,
        });

        assert_eq!(book.best_bid(), Some(105)); // Highest bid wins
        assert_eq!(book.best_ask(), Some(108)); // Lowest ask wins
    }
}