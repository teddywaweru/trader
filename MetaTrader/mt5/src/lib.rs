#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod account;
mod error;
mod indicator;
mod instant_rates;
mod mt5_bridge;
mod ohlc;
mod order;
mod parse;
mod sockets;
mod symbol;
mod tick;
mod timeframe;
mod trade;

pub use account::Account;
pub use indicator::Indicator;
pub use instant_rates::InstantRates;
use mt5_bridge::Mt5Bridge;
pub use ohlc::OHLC;
use order::{serde_order_type, serde_order_type_filling};
pub use order::{Order, OrderRequest, OrderType, OrderTypeFilling};
pub use sockets::ConnectionSockets;
pub use symbol::Symbol;
pub use tick::{HistoricalTickData, HistoricalTickDataRequest};
pub use timeframe::{serde_timeframe, Timeframe};
//
//
pub use trade::OpenTrade;
// use serde::{Deserialize, Serialize};
// use serde_json::{Map, Value};
//
// enum Actions {
//     // GET_CURRENT_RATE("GET_CURRENT_RATE"),
//     // GET_INDICATOR_DATA("GET_INDICATOR_DATA"),
//     GetCurrentRate(String),
//     GetIndicatorData(String),
// }
