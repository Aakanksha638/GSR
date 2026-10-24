
use gsr::{Order, OrderBook, Side}; // Import the reusable engine from the library.

fn main() { // Start the demo program.
    let mut book = OrderBook::new(); // Create an empty order book.

    book.add_order(Order { id: 1, side: Side::Sell, price: 100, quantity: 5 }); // Add a seller offering five units at 100.

    let trades = book.add_order(Order { id: 2, side: Side::Buy, price: 101, quantity: 3 }); // Buy three units at a price up to 101.

    for trade in trades { // Process each executed trade.
        println!("{:?}", trade); // Print the trade details.
    } // Finish printing trades.

    println!("Best bid: {:?}", book.best_bid()); // Print the highest buy price.
    println!("Best ask: {:?}", book.best_ask()); // Print the lowest sell price.
} // Finish the demo.
