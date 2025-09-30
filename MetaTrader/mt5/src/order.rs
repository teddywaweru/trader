use crate::mt5_bridge::Mt5Bridge;
use crate::{
    error::Mt5Error, Account, HistoricalTickDataRequest, Indicator, Symbol, Timeframe,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use strum::{EnumIter, IntoEnumIterator};

/// Order Object Data for Generating New Requests to the MT5 Server
/// Implemented for:
/// - New Order Generation
/// - Modifying existing Trades
/// - Closing existing Trades
/// -
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequest {
    pub account: Account,
    pub order_type: OrderType,
    pub symbol: Symbol,
    pub risk: f32,
    pub limit: Option<u32>, // volume: f32,
                            // price: f32,
                            // sl: f32,
                            // tp: f32,
                            // order_type_filling: OrderTypeFilling,
                            // order_type_time: DateTime<Utc>,
                            // comment: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub magic: u32,
    pub ticket: u32,
    pub symbol: Symbol,
    pub volume: f32,
    pub price: f32,
    pub sl: f32,
    pub tp: f32,
    pub deviation: u32,
    pub order_type: OrderType,
    pub order_type_filling: OrderTypeFilling,
    pub order_type_time: OrderTypeTime,
    pub expiration: DateTime<Utc>,
    pub comment: String,
    pub position: u32,
    pub position_by: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderTypeFilling {
    OrderFillingFOK = 0,
    OrderFillingIOC = 1,
    OrderFillingBOC = 2,
    OrderFillingReturn = 3, // SymbolTradeExecutionInstant,
                            // SymbolTradeExecutionRequest,
                            // SymbolTradeExecutionMarket,
                            // SymbolTradeExecutionExchange,
}

pub mod serde_order_type_filling {
    use crate::OrderTypeFilling;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OrderTypeFilling, D::Error>
    where
        D: Deserializer<'de>,
    {
        let type_filling = u8::deserialize(deserializer)?;
        let type_filling = match type_filling {
            0 => OrderTypeFilling::OrderFillingFOK,
            1 => OrderTypeFilling::OrderFillingIOC,
            2 => OrderTypeFilling::OrderFillingBOC,
            3 => OrderTypeFilling::OrderFillingReturn,
            _ => panic!("No matching Type Filling for val provided: \n Value: {type_filling}"),
        };

        Ok(type_filling)
    }
    pub fn serialize<S>(type_filling: &OrderTypeFilling, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let type_filling = match type_filling {
            OrderTypeFilling::OrderFillingFOK => OrderTypeFilling::OrderFillingFOK as u8,
            OrderTypeFilling::OrderFillingIOC => OrderTypeFilling::OrderFillingIOC as u8,
            OrderTypeFilling::OrderFillingBOC => OrderTypeFilling::OrderFillingBOC as u8,
            OrderTypeFilling::OrderFillingReturn => OrderTypeFilling::OrderFillingReturn as u8,
        };

        serializer.serialize_u8(type_filling)
    }
}

impl Default for OrderTypeFilling {
    fn default() -> Self {
        Self::OrderFillingIOC
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum OrderTypeTime {
    OrderTimeGtc,
    OrderTimeDay,
    OrderTimeSpecified,
    OrderTimeSpecifiedDay,
}

impl Default for OrderTypeTime {
    fn default() -> Self {
        Self::OrderTimeGtc
    }
}

#[derive(Debug, EnumIter, Serialize, Deserialize)]
pub enum OrderType {
    OrderTypeBuy = 0,
    OrderTypeSell = 1,
    OrderTypeBuyLimit = 2,
    OrderTypeSellLimit = 3,
    OrderTypeBuyStop = 4,
    OrderTypeSellStop = 5,
    OrderTypeBuyStopLimit = 6,
    OrderTypeSellStopLimit = 7,
    OrderNanDefault,
}
impl Default for OrderType {
    fn default() -> Self {
        Self::OrderNanDefault
    }
}
impl OrderType {
    pub fn list_order_types() -> Vec<OrderType> {
        OrderType::iter().collect()
    }
}
impl From<&str> for OrderType {
    fn from(value: &str) -> Self {
        match value {
            "buy" => OrderType::OrderTypeBuy,
            "sell" => OrderType::OrderTypeSell,
            "buylimit" => OrderType::OrderTypeBuyLimit,
            "selllimit" => OrderType::OrderTypeSell,
            "buystop" => OrderType::OrderTypeBuyStop,
            "sellstop" => OrderType::OrderTypeSellStop,
            _ => panic!("Unable to translate the OrderType String to Object: {value}"),
        }
    }
}

pub mod serde_order_type {
    use crate::OrderType;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OrderType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let order_type = u8::deserialize(deserializer)?;

        let order_type = match order_type {
            0 => OrderType::OrderTypeBuy,
            1 => OrderType::OrderTypeSell,
            2 => OrderType::OrderTypeBuyLimit,
            3 => OrderType::OrderTypeSellLimit,
            4 => OrderType::OrderTypeBuyStop,
            5 => OrderType::OrderTypeSellStop,
            6 => OrderType::OrderTypeBuyStopLimit,
            7 => OrderType::OrderTypeSellStopLimit,
            _ => OrderType::OrderNanDefault,
        };

        Ok(order_type)
    }
    pub fn serialize<S>(order_type: &OrderType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let order_type = match order_type {
            OrderType::OrderTypeBuy => OrderType::OrderTypeBuy as u8,
            OrderType::OrderTypeSell => OrderType::OrderTypeSell as u8,
            OrderType::OrderTypeBuyLimit => OrderType::OrderTypeBuyLimit as u8,
            OrderType::OrderTypeSellLimit => OrderType::OrderTypeSellLimit as u8,
            OrderType::OrderTypeBuyStop => OrderType::OrderTypeBuyStop as u8,
            OrderType::OrderTypeSellStop => OrderType::OrderTypeSellStop as u8,
            OrderType::OrderTypeBuyStopLimit => OrderType::OrderTypeBuyStopLimit as u8,
            OrderType::OrderTypeSellStopLimit => OrderType::OrderTypeSellStopLimit as u8,
            OrderType::OrderNanDefault => OrderType::OrderNanDefault as u8,
        };

        serializer.serialize_u8(order_type)
    }
}
impl Default for Order {
    fn default() -> Self {
        Order {
            magic: 123321,
            ticket: 0,
            symbol: Symbol::default(),
            volume: 0.01,
            price: 0.0,
            sl: 500.0,
            tp: 500.0,
            deviation: 0,
            order_type: OrderType::OrderNanDefault,
            order_type_filling: OrderTypeFilling::default(),
            order_type_time: OrderTypeTime::default(),
            expiration: Utc::now(),
            comment: "Default Trade".to_string(),
            position: 0,
            position_by: 0,
        }
    }
}

impl Default for OrderRequest {
    fn default() -> Self {
        OrderRequest {
            account: Account::default(),
            risk: 0.02,
            symbol: Symbol::default(),
            // volume: 0.01,
            // price: 30.0,
            // sl: 5.0,
            // tp: 5.0,
            order_type: OrderType::default(),
            limit: None,
        }
    }
}
impl OrderRequest {}
//Calculate Order parameters, which generates an order request
//Send an order request, which generates and executes an order
impl From<&OrderRequest> for Order {
    fn from(request: &OrderRequest) -> Self {
        Order {
            magic: todo!(),
            ticket: todo!(),
            symbol: todo!(),
            volume: todo!(),
            price: todo!(),
            sl: todo!(),
            tp: todo!(),
            deviation: todo!(),
            order_type: todo!(),
            order_type_filling: todo!(),
            order_type_time: todo!(),
            expiration: todo!(),
            comment: todo!(),
            position: todo!(),
            position_by: todo!(),
        }
    }
}
impl Order {
    // pub fn new_order(symbol: Symbol, order_type: OrderType, risk: f32, account: Account) -> Self {
    pub fn new_order(order_request: OrderRequest) -> Self {
        // let request = order_request;
        // let risk_amount = Account::from(order_request.account.as_str()).current_balance * order_request.risk;
        let risk_amount = order_request.account.current_balance * order_request.risk;

        // let account = Mt5Bridge::get_account_info();
        let pip_value: f32;
        let volume: f32;
        let sl: f32;
        let tp: f32;
        let pips: u32 = order_request.limit.unwrap();
        let tp_multiplier: f32 = 1.0;
        let sl_multiplier: f32 = 1.5;
        let price: f32;
        let comment: String;
        let magic: u32 = 0;

        // CALCULATING LOT SIZE -> get pip value first
        // get instrument category
        // separate symbol to first and second currencies
        match order_request.order_type {
            OrderType::OrderTypeBuy => {
                price = order_request.symbol.ask;
                sl = order_request.symbol.bid
                    - ((pips * 10) as f32 * order_request.symbol.point * sl_multiplier) as f32;
                tp = price
                    + ((pips * 10) as f32 * order_request.symbol.point * tp_multiplier) as f32;
                comment = "testing".to_string();
                volume = risk_amount
                    / (order_request.symbol.tick_value * 10 as f32 * pips as f32 * sl_multiplier)
                        as f32;
            }
            OrderType::OrderTypeSell => {
                price = order_request.symbol.bid;
                sl = order_request.symbol.ask
                    + ((pips * 10) as f32 * order_request.symbol.point * sl_multiplier) as f32;
                tp = price
                    - ((pips * 10) as f32 * order_request.symbol.point * tp_multiplier) as f32;
                comment = "testing sells".to_string();
                volume = risk_amount
                    / (order_request.symbol.tick_value * 10 as f32 * pips as f32 * sl_multiplier)
                        as f32;
            }
            OrderType::OrderTypeBuyLimit => {
                panic!()
            }
            _ => {
                panic!()
            }
        }

        let mut new_order = Order::default();
        new_order.order_type = order_request.order_type;
        new_order.symbol = order_request.symbol;
        new_order.price = price;
        new_order.sl = sl;
        new_order.tp = tp;
        new_order.comment = comment;
        new_order.volume = volume;
        new_order.magic = magic;

        new_order
    }
    fn calc_pip_value(symbol: &Symbol, account_curr: &str) -> f32 {
        let pip_value: f32;
        let (base_curr, quote_curr) = symbol.name.split_at(3);
        if account_curr == base_curr {
            match quote_curr {
                "JPY" => {
                    pip_value = symbol.bid.recip() * 100 as f32;
                }
                _ => {
                    pip_value = symbol.bid.recip() as f32;
                }
            }
        } else if account_curr == quote_curr {
            pip_value = 1.0;
        } else {
            // use crate::ConnectionSockets;
            // let sockets = ConnectionSockets::initialize().unwrap();
            let data = "DATA;GET_SYMBOLS";
            println!("quote_curr: {quote_curr}");
            //
            //FIX: Source of mt5 string?
            let symbols = Symbol::get_all("mt5");
            let alt_symbol: &Symbol = symbols
                .iter()
                .filter(|symbol| {
                    let (alt_base, alt_quote) = symbol.name.split_at(3);
                    if alt_base == account_curr || alt_quote == account_curr {
                        true
                    } else {
                        false
                    }
                })
                .filter(|symbol| {
                    let (alt_base, alt_quote) = symbol.name.split_at(3);
                    if alt_base == quote_curr || alt_quote == quote_curr {
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<&Symbol>>()
                .first()
                .unwrap();

            if &alt_symbol.name[0..3] == account_curr {
                pip_value = 1 as f32 / alt_symbol.bid;
            } else {
                pip_value = 1 as f32 * alt_symbol.bid;
            }
        }

        pip_value
    }
    pub fn from_mt5_response(data: &str) -> Self {
        use crate::error::Mt5Error;
        use serde_json::Value;
        if let Ok(data) = serde_json::from_str::<Order>(data) {
            return data;
        } else {
            println!("Data received on mt5_response for new order: {data}");
            match serde_json::from_str::<Value>(data)
                .unwrap()
                // .get("error_kind")
                // .unwrap()
                // .to_owned()
                .to_string()
                .as_str()
            {
                "No Connection" => todo!(),
                _ => todo!(),
            }
            if let Ok(data) = serde_json::from_str::<Mt5Error>(data) {
                panic!(
                    "Response from MetaTrader: Error in Execution\n. Response {:#?}",
                    data
                )
            } else {
                panic!(
                    "Unable to parse response to serde_json Value Object. \n Received Response: \n{}",
                    data)
            }
        }
    }
    pub fn execute_order(self, bridge: &str) -> String {
        match bridge {
            "mt5" => {
                Mt5Bridge::request_order(self);
            }
            &_ => {
                todo!()
            }
        }
        todo!()
    }

    fn calculate_pips(symbol: &Symbol) -> u32 {
        let timeframe = Timeframe::default();
        // 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME
        // NOTE: datetime has format: D'2015.01.01 00:00'
        //
        let period = 40;
        let timeframe = Timeframe::D1;
        // let start_date = Mt5Bridge::get_historical_tick_data(timeframe.into());
        let end_date = Utc::now();
        let duration = Duration::days(period);
        let start_date = end_date - duration;
        let historical_data_request =
            HistoricalTickDataRequest::new(&symbol, timeframe, start_date, end_date)
                .to_mt5_request();
        todo!()
        // let atr = Indicator::get_atr(&historical_data.ticks);
        // let atr = atr / symbol.point / 10 as f32;

        // atr as u32
    }
}

#[cfg(test)]
mod trade_tests {
    use super::*;

    #[test]
    fn test_calculate_pips() {
        let timeframe = Timeframe::default();
        // 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME
        //
        let period = 14;
        let symbol = Symbol::default();
        let end_date = Utc::now();
        let duration = Duration::days(period);
        let start_date = end_date - duration;
        let historical_data_request =
            HistoricalTickDataRequest::new(symbol, timeframe, start_date, end_date);
        let data = historical_data_request.to_mt5_request();
        todo!()

        // let response = Mt5Bridge::get_historical_tick_data(historical_data_request);
        // let tick_data = Mt5Bridge::get_historical_tick_data(timeframe.into());
    }
}
