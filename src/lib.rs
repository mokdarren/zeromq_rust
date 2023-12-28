use rand::Rng;
use serde::{Deserialize, Serialize};

pub const PUSH_PORT: i32 = 7020;

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeEvent {
    pub symbol: String,
    pub quantity: f64,
    pub side: String,
    pub order_type: String,
}

impl TradeEvent {
    pub fn new(symbol: String, quantity: f64, side: String, order_type: String) -> TradeEvent {
        TradeEvent {
            symbol,
            quantity,
            side,
            order_type,
        }
    }

    pub fn new_random_trade() -> TradeEvent {
        let mut rng = rand::thread_rng();
        let symbols = vec!["btcusdt", "ethusdt"];
        let sides = vec!["BUY", "SELL"];
        let order_types = vec!["MARKET", "LIMIT"];

        let symbol = symbols[rng.gen_range(0..symbols.len())];
        let quantity = rng.gen_range(0.01f64..1.0f64);
        let side = sides[rng.gen_range(0..sides.len())];
        let order_type = order_types[rng.gen_range(0..order_types.len())];

        TradeEvent {
            symbol: symbol.to_string(),
            quantity: quantity,
            side: side.to_string(),
            order_type: order_type.to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
