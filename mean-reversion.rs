use std::collections::VecDeque;

// Struct to hold state of the mean reversion strategy
struct MeanReversion {
    window_size: usize,
    prices: VecDeque<f64>,
    mean: f64,
    threshold: f64,
}

impl MeanReversion {
    // Initialize with window size and threshold
    fn new(window_size: usize, threshold: f64) -> Self {
        MeanReversion {
            window_size,
            prices: VecDeque::with_capacity(window_size),
            mean: 0.0,
            threshold,
        }
    }

    // Add a new price to the history
    fn add_price(&mut self, price: f64) {
        self.prices.push_back(price);
        if self.prices.len() > self.window_size {
            self.prices.pop_front();
        }
    }

    // Calculate the current mean of the prices in the window
    fn calculate_mean(&self) -> f64 {
        let sum: f64 = self.prices.iter().sum();
        sum / self.prices.len() as f64
    }

    // Check if the current price is below the mean by the threshold
    fn should_buy(&self, current_price: f64) -> bool {
        self.mean - current_price > self.threshold
    }

    // Check if the current price is above the mean by the threshold
    fn should_sell(&self, current_price: f64) -> bool {
        current_price - self.mean > self.threshold
    }

    // Update the mean and make trading decisions based on the current price
    fn trade_decision(&mut self, current_price: f64) -> Option<&str> {
        self.add_price(current_price);
        self.mean = self.calculate_mean();

        if self.should_buy(current_price) {
            Some("Buy")
        } else if self.should_sell(current_price) {
            Some("Sell")
        } else {
            None
        }
    }
}

fn main() {
    // Example usage
    let mut strategy = MeanReversion::new(10, 0.2); // Using a window of 10 prices and a threshold of 0.2
    let prices = vec![100.0, 95.0, 110.0, 105.0, 98.0, 102.0, 97.0, 103.0, 100.0, 98.0, 105.0];

    for price in prices {
        match strategy.trade_decision(price) {
            Some(action) => println!("Action: {} at price {}", action, price),
            None => (),
        }
    }
}
