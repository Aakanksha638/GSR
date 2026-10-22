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

    
fn add_order(&mut self, mut incoming: Order) -> Vec<Trade> { // Process an incoming order.
    let mut trades = Vec::new(); // Store every trade created by this order.

    match incoming.side { // Choose the matching logic based on the incoming side.
        Side::Buy => { // A buyer matches against the lowest-priced sellers.
            while incoming.quantity > 0 { // Continue while the buyer has quantity left.
                let best_ask = match self.asks.keys().next().copied() { // Find the cheapest ask.
                    Some(price) => price, // Use the best available ask price.
                    None => break, // Stop when there are no sellers.
                }; // Finish finding the best ask.

                if incoming.price < best_ask { // Check whether the buyer can afford the ask.
                    break; // Stop if the prices do not cross.
                } // Finish the price check.

                let (traded_quantity, filled_id, level_empty) = { // Limit the mutable borrow to this block.
                    let queue = self.asks.get_mut(&best_ask).unwrap(); // Get sellers at this price.
                    let maker = queue.front_mut().unwrap(); // FIFO means the oldest seller goes first.
                    let traded_quantity = incoming.quantity.min(maker.quantity); // Trade the smaller quantity.
                    maker.quantity -= traded_quantity; // Reduce the resting seller's quantity.
                    incoming.quantity -= traded_quantity; // Reduce the buyer's remaining quantity.
                    let filled_id = if maker.quantity == 0 { Some(maker.id) } else { None }; // Remember a fully filled seller.
                    if filled_id.is_some() { queue.pop_front(); } // Remove the seller if completely filled.
                    (traded_quantity, filled_id, queue.is_empty()) // Return cleanup information.
                }; // Release the mutable borrow before updating other fields.

                trades.push(Trade { // Record the completed trade.
                    maker_order_id: filled_id.unwrap_or_else(|| { // Use the resting seller ID when it remains.
                        self.asks.get(&best_ask).unwrap().front().unwrap().id // Read the remaining seller ID.
                    }), // Finish selecting the maker ID.
                    taker_order_id: incoming.id, // Record the incoming buyer ID.
                    price: best_ask, // Trades execute at the resting seller's price.
                    quantity: traded_quantity, // Record the traded quantity.
                }); // Finish recording the trade.

                if let Some(id) = filled_id { self.order_prices.remove(&id); } // Remove a filled seller from the index.
                if level_empty { self.asks.remove(&best_ask); } // Remove an empty price level.
            } // Finish matching the incoming buyer.

            if incoming.quantity > 0 { // Keep any unfilled buyer quantity in the book.
                self.order_prices.insert(incoming.id, (Side::Buy, incoming.price)); // Index the resting buy order.
                self.bids.entry(incoming.price).or_default().push_back(incoming); // Store it on the bid side.
            } // Finish storing the remaining buyer quantity.
        } // Finish buy-side handling.

        Side::Sell => { // A seller matches against the highest-priced buyers.
            while incoming.quantity > 0 { // Continue while the seller has quantity left.
                let best_bid = match self.bids.keys().next_back().copied() { // Find the highest bid.
                    Some(price) => price, // Use the best available bid price.
                    None => break, // Stop when there are no buyers.
                }; // Finish finding the best bid.

                if incoming.price > best_bid { // Check whether the bid meets the seller's price.
                    break; // Stop if the prices do not cross.
                } // Finish the price check.

                let (maker_id, traded_quantity, filled_id, level_empty) = { // Limit the mutable borrow to this block.
                    let queue = self.bids.get_mut(&best_bid).unwrap(); // Get buyers at this price.
                    let maker = queue.front_mut().unwrap(); // FIFO means the oldest buyer goes first.
                    let maker_id = maker.id; // Save the buyer ID before possibly removing it.
                    let traded_quantity = incoming.quantity.min(maker.quantity); // Trade the smaller quantity.
                    maker.quantity -= traded_quantity; // Reduce the resting buyer's quantity.
                    incoming.quantity -= traded_quantity; // Reduce the seller's remaining quantity.
                    let filled_id = if maker.quantity == 0 { Some(maker.id) } else { None }; // Remember a fully filled buyer.
                    if filled_id.is_some() { queue.pop_front(); } // Remove the buyer if completely filled.
                    (maker_id, traded_quantity, filled_id, queue.is_empty()) // Return trade and cleanup information.
                }; // Release the mutable borrow before updating other fields.

                trades.push(Trade { // Record the completed trade.
                    maker_order_id: maker_id, // Record the resting buyer ID.
                    taker_order_id: incoming.id, // Record the incoming seller ID.
                    price: best_bid, // Trades execute at the resting buyer's price.
                    quantity: traded_quantity, // Record the traded quantity.
                }); // Finish recording the trade.

                if let Some(id) = filled_id { self.order_prices.remove(&id); } // Remove a filled buyer from the index.
                if level_empty { self.bids.remove(&best_bid); } // Remove an empty price level.
            } // Finish matching the incoming seller.

            if incoming.quantity > 0 { // Keep any unfilled seller quantity in the book.
                self.order_prices.insert(incoming.id, (Side::Sell, incoming.price)); // Index the resting sell order.
                self.asks.entry(incoming.price).or_default().push_back(incoming); // Store it on the ask side.
            } // Finish storing the remaining seller quantity.
        } // Finish sell-side handling.
    } // Finish choosing the incoming order's side.

    trades // Return all trades created by this order.
} // Finish adding an order.


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

    #[test] // Tell Rust to run this test
fn fully_filled_order_is_removed_from_index() {
    let mut book = OrderBook::new(); // Create an empty book

    book.add_order(Order { // Add a resting sell order
        id: 10,
        side: Side::Sell,
        price: 100,
        quantity: 2,
    });

    assert!(book.order_prices.contains_key(&10)); // Confirm it is indexed

    let trades = book.add_order(Order { // Buy the entire resting quantity
        id: 20,
        side: Side::Buy,
        price: 100,
        quantity: 2,
    });

    assert_eq!(trades.len(), 1); // Exactly one trade should execute
    assert_eq!(trades[0].quantity, 2); // Both units should be filled
    assert!(!book.order_prices.contains_key(&10)); // Filled order leaves the index
    assert_eq!(book.best_ask(), None); // No sell order should remain
}
}