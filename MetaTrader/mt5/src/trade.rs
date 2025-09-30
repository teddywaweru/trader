use crate::serde_order_type;
use crate::{OrderType, Symbol};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenTrade {
    ticket: u32,
    magic: i32,
    symbol: Symbol,
    lot_size: f32,
    #[serde(with = "serde_order_type")]
    trade_type: OrderType,
    open_price: f32,
    open_time: String,
    sl: f32,
    tp: f32,
    pnl: f32,
    comment: String,
}
impl Default for OpenTrade {
    fn default() -> Self {
        Self {
            ticket: 123321,
            magic: 123321,
            symbol: Symbol::default(),
            lot_size: 0.01,
            trade_type: OrderType::OrderNanDefault,
            open_price: 0.0,
            open_time: "Default time".to_string(),
            sl: 500.0,
            tp: 500.0,
            pnl: 0.0,
            comment: "Default Comment".to_string(),
        }
    }
}
impl OpenTrade {
    pub fn from_mt5_response(data: &str) -> Self {
        let data = data.replace("'", "\"");
        serde_json::from_str(&data).expect(&format!(
            "Unable to parse string to MT5 OpenTrades Object:\n Received String:\n {}",
            data
        ))
    }
}
impl OpenTrade {
    pub fn get_all(bridge: &str) -> Result<Vec<OpenTrade>, Box<dyn std::error::Error>> {
        match bridge {
            "mt5" => Mt5Bridge::get_open_trades(),
            &_ => {
                todo!()
            }
        }
    }
}

struct ClosedTrade {
    ticket: u32,
    magic: i32,
    symbol: Symbol,
    lot_size: f32,
    trade_type: OrderType,
    open_price: f32,
    close_price: f32,
    open_time: String,
    stop_loss: f32,
    take_profit: f32,
    pnl: f32,
    comment: String,
}
