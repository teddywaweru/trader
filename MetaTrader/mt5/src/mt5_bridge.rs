// requesting for:
// Existing trades
// Tick values
// Account Info
use crate::{
    account::Account,
    error::Mt5Error,
    ohlc::OHLC,
    order::{FilledOrder, Order},
    sockets::ConnectionSockets,
    symbol::Symbol,
    tick::HistoricalTickData,
    timeframe::Timeframe,
    trade::OpenTrade,
    HistoricalTickDataRequest,
};
// use crate::{
//     Account, HistoricalTickData, HistoricalTickDataRequest, InstantRates, OpenTrade, Order,
//     OrderRequest, OrderType, Symbol, Symbols, Timeframe,
// };
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
//TRADES

pub struct Mt5Bridge {
    sockets: ConnectionSockets,
}

// Execute Operations
impl Mt5Bridge {
    pub fn request_order(order: &Order) -> Result<FilledOrder, Box<dyn std::error::Error>> {
        let bridge = Self::init();
        let data = format!(
            "TRADE;OPEN;{:#?};{};{};{};{};{};{:.02};{};{},{:#?}",
            order.order_type as u8,
            order.symbol.name,
            order.open_price,
            order.sl,
            order.tp,
            order.comment,
            order.volume,
            order.magic,
            order.ticket,
            order.order_type_filling
        );
        let response = bridge.sockets.request(&data, 0).receive();

        match bridge.parse_order_data(response.as_str()) {
            Ok(response) => return Ok(response),
            Err(response) => {
                panic!("Here's the response{:?} ", response)
            }
        }

        fn modify_trade() {
            todo!()
        }
        fn close_trade() {
            todo!()
        }
    }
}

//  INIT connection sockets
impl Mt5Bridge {
    pub fn init() -> Self {
        let sockets = ConnectionSockets::initialize().unwrap();
        Mt5Bridge { sockets }
    }
}
// Collect Data Reports
impl Mt5Bridge {
    pub fn get_instant_rates(symbol: &str) -> String {
        let bridge = Mt5Bridge::init();
        let data = "DATA; GET_INSTANT_RATES";

        let response = bridge.sockets.request(data, 0).receive();

        response
    }
    pub fn get_historical_tick_data(
        symbol: &str,
        timeframe: &str,
        start_date: u32,
        end_date: u32,
    ) -> HistoricalTickData {
        let bridge = Mt5Bridge::init();

        // TODO: Prepare data request
        let data = &format!(
            "DATA;GET_HISTORICAL_DATA;{};{};{};{}",
            symbol, timeframe, start_date, end_date
        );

        let response = bridge.sockets.request(data, 0).receive();
        let response = bridge.parse_historical_tick_data(&response);

        response
    }

    pub fn get_indicator_data(data: &str) -> String {
        todo!()
    }
    pub fn get_open_trades() -> Result<Vec<OpenTrade>, Box<dyn std::error::Error>> {
        let bridge = Self::init();

        let data = "TRADE;GET_OPEN_TRADES";
        let response = bridge.sockets.request(data, 0).receive();
        bridge.parse_open_trades(&response)
    }

    pub fn mt5_date_from(date: chrono::DateTime<Utc>) -> String {
        let date = std::time::Instant::now();
        let date = format!("{:#?}", date);
        println!("Here's the date: {date}");
        todo!()
    }
}

//SYMBOLS
impl Mt5Bridge {
    pub fn get_symbols() -> Vec<Symbol> {
        let data = "DATA;GET_SYMBOLS";
        let bridge = Self::init();
        let response = bridge.sockets.request(data, 1).receive();

        bridge.parse_get_symbols(response)
    }
    pub fn get_symbol_data(symbol: &str) -> Symbol {
        let data = format!("DATA;GET_SYMBOL_DATA;{}", symbol);
        let bridge = Self::init();
        let response = bridge.sockets.request(&data, 1).receive();
        bridge.parse_get_symbol_data(&response)
    }
}

//ACCOUNT
impl Mt5Bridge {
    pub fn get_account_info() -> Account {
        let bridge = Self::init();
        let data = "DATA;GET_ACCOUNT_INFO;";

        let response = bridge.sockets.request(data, 0).receive();

        bridge.parse_account_info(&response)
    }
}

// PARSER
impl Mt5Bridge {
    fn parse_account_info(&self, response: &str) -> Account {
        let mut response = self.sanitize_mt5_response(&response);
        let response = response.remove("account_info").unwrap();
        serde_json::from_value::<Account>(response).unwrap()
    }
    fn parse_get_symbols(&self, data: String) -> Vec<Symbol> {
        //NOTE: Data Received Takes the form {"action":"...", "symbols": "..."}
        let mut data = self.sanitize_mt5_response(&data);

        //NOTE: Removing to avoid cloning. Data is not needed anywhere else.
        let data = data.remove("symbols").unwrap();
        let data = serde_json::from_value::<Vec<String>>(data).unwrap();

        let mut symbols: Vec<Symbol> = vec![];
        for symbol_name in data {
            let mut symbol = Symbol::default();
            symbol.name = symbol_name;
            symbols.push(symbol);
        }
        symbols
    }
    fn parse_get_symbol_data(&self, data: &str) -> Symbol {
        let mut data = self.sanitize_mt5_response(data);
        let data = data.remove("symbol").unwrap();
        serde_json::from_value::<Symbol>(data).unwrap()
    }
    fn parse_historical_tick_data(&self, data: &str) -> HistoricalTickData {
        let mut data = self.sanitize_mt5_response(data);

        let timeframe = data.remove("timeframe").unwrap();
        let ticks = data.remove("ticks").unwrap();
        HistoricalTickData {
            timeframe: serde_json::from_value::<Timeframe>(timeframe).unwrap(),
            ticks: serde_json::from_value::<Vec<OHLC>>(ticks).unwrap(),
        }
    }
    fn parse_order_data(&self, response: &str) -> Result<FilledOrder, Box<dyn std::error::Error>> {
        let mut response = self.sanitize_mt5_response(response);
        let response = response
            .remove("result")
            .ok_or(Mt5Error::from_mt5_response(
                "No result value in response found during parsing",
            ))?;
        let filled_order =
            serde_json::from_value::<FilledOrder>(response.clone()).map_err(|e| {
                Mt5Error::new(
                    &format!("Unable to serde_json parse response \n {response:#?}"),
                    e,
                )
            })?;
        Ok(filled_order)

    }
    fn parse_open_trades(
        &self,
        response: &str,
    ) -> Result<Vec<OpenTrade>, Box<dyn std::error::Error>> {
        let mut response = self.sanitize_mt5_response(response);
        if let Some(response) = response.remove("trades") {
            // panic!("Here's the response, {:?}", response);
            match serde_json::from_value::<Vec<OpenTrade>>(response) {
                Ok(r) => {
                    return Ok(r);
                }
                Err(r) => return Err(Box::new(r)),
            }
        } else {
            return Err(format!(
                "No Open Trade Positions obtained in Response. MT5 Response: \n{:?}",
                response
            )
            .into());
        }
    }
    // Replace single quotations with double for parsing with serde_json
    // Remove the action key term located in almost every request
    fn sanitize_mt5_response(&self, response: &str) -> serde_json::Map<String, Value> {
        let response = response.replace("'", "\"");
        let response = serde_json::from_str(&response).expect(&format!(
            "Unable to parse string to Map<String, Value>\n Received String: \n {}",
            response
        ));
        response
    }
}
