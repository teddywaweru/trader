use crate::{
    mt5_bridge::Mt5Bridge, serde_order_type_filling, HistoricalTickData, OrderTypeFilling,
    Timeframe,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub sector: String,
    pub spread: u32,
    pub point: f32,
    pub bid: f32,
    pub ask: f32,
    pub tick_value: f32,
    #[serde(with = "serde_order_type_filling")]
    pub type_filling: OrderTypeFilling,
}

impl Default for Symbol {
    fn default() -> Self {
        let name = "EURUSD_default".to_string();
        let sector = "Default Category".to_string();
        let spread = 3;
        let point = 5.33;
        let bid = 234.23;
        let ask = 234.2;
        let tick_value = 33.3;
        let type_filling = OrderTypeFilling::default();
        Symbol {
            name,
            sector,
            spread,
            point,
            bid,
            ask,
            tick_value,
            type_filling,
        }
    }
}
impl Symbol {
    pub fn get_all(bridge: &str) -> Vec<Symbol> {
        match bridge {
            "mt5" => Mt5Bridge::get_symbols(),
            &_ => {
                todo!()
            }
        }
    }
    pub fn parse_mt5_response(data: &str) -> Self {
        let symbol = match serde_json::from_str(&data) {
            Ok(symbol) => symbol,
            Err(e) => {
                panic!("Unable to parse string to Symbol object. \n Received String: \n {data} \n Error: {e}")
            }
        };
        println!("Symbol Received from mt5: {:#?}", symbol);

        symbol
    }
    pub fn get_symbol_data(bridge: &str, symbol: &str) -> Symbol {
        match bridge {
            "mt5" => Mt5Bridge::get_symbol_data(symbol),
            &_ => {
                todo!()
            }
        }
    }
    pub fn get_historical_tick_data(
        &self,
        bridge: &str,
        timeframe: &str,
        start: u32,
        end: u32,
    ) -> HistoricalTickData {
        match bridge {
            "mt5" => Mt5Bridge::get_historical_tick_data(&self.name, timeframe, start, end),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod SymbolTests {
    use super::*;

    #[test]
    fn get_symbol_info_test() {}
}
