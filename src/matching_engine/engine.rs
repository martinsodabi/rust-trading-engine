use std::collections::HashMap;

use rust_decimal::Decimal;

use super::orderbook::{Order, Orderbook};

// BTCUSD
// BTC => BASE
// USD => QUOTE
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        return TradingPair { base, quote };
    }

    pub fn to_string(self) -> String {
        return format!("{}_{}", self.base, self.quote);
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        return MatchingEngine {
            orderbooks: HashMap::new(),
        };
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("Placed limit order at price level {:?}", price);
                return Ok(());
            }
            None => {
                return Err(format!(
                    "The Orderbook for the given trading pair ({}) does not exist!",
                    pair.to_string()
                ));
            }
        }
    }
}
