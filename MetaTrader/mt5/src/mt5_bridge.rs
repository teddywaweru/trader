// requesting for:
// Existing trades
// Tick values
// Account Info
use crate::{
    account::Account,
    ohlc::OHLC,
    order::Order,
    sockets::ConnectionSockets,
    symbol::Symbol,
    tick::HistoricalTickData,
    timeframe::Timeframe,
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

// Get generic information from mt5
// impl Mt5Bridge {
//     //TICK DATA
//     fn get_tick_data() {
//         todo!()
//     }
//     fn get_ohlc_data() {
//         todo!()
//     }
//     pub fn get_symbol_info(symbol: &str) -> Symbol {
//         // TODO only get single symbol, instead of array
//         // let symbols = Self::get_symbols();
//         let data = format!("DATA;GET_SYMBOL_INFO;{}", symbol);
//         let response = Self::init().sockets.request(&data, 0).receive();
//         let symbol = Symbol::parse_mt5_response(&response);
//
//         symbol
//     }
//     pub fn get_symbols() -> Symbols {
//         let data = "DATA;GET_SYMBOLS";
//         let response = Mt5Bridge::init().sockets.request(data, 1).receive();
//
//         let response = Symbols::parse_mt5_response(&response);
//         response
//     }
// }

// Execute Operations
impl Mt5Bridge {
    pub fn request_order(order: Order) -> Order {
        let bridge = Self::init();
        let data = format!(
            "TRADE;OPEN;{:#?};{};{};{};{};{};{:.02};{};{},{:#?}",
            order.order_type as u8,
            order.symbol.name,
            order.price,
            order.sl,
            order.tp,
            order.comment,
            order.volume,
            order.magic,
            order.ticket,
            order.order_type_filling
        );
        let response = bridge.sockets.request(&data, 0).receive();

        bridge.parse_order_data(response.as_str()).unwrap()

        //lot size
        //tp and sl values
        //risk amount
    }
    // pub fn generate_order(symbol: &str, order_type: &str, risk: f32) -> OpenTrade {
    //     let account = Self::get_account_info();
    //     let symbol = Self::get_symbol_info(symbol);
    //
    //     //Only processing trades in currency pairs only currently
    //     if symbol.sector != "Currency" {
    //         panic!(
    //             "Unable to generate trades for symbols that are not in the Currency sector. \n
    //               Received symbol: {symbol:#?}"
    //         );
    //     }
    //
    //     //Static OrderType currently
    //     let order_type = OrderType::from(order_type);
    //     let order_request = OrderRequest {
    //         account: Account::default(),
    //         order_type,
    //         symbol,
    //         risk
    //         // volume: todo!(),
    //         // price: todo!(),
    //         // sl: todo!(),
    //         // tp: todo!(),
    //     };
    //     // let mut order_request = OrderRequest::default();
    //     let request = Order::new_order(order_request).generate_request();
    //     // let request = Order::new_order(symbol, order_type, risk, account).generate_request();
    //
    //     let response = Mt5Bridge::init().sockets.request(&request, 0).receive();
    //     let response = OpenTrade::from_mt5_response(&response);
    //
    //     println!("Response back on OPEN_TRADE:\n {:#?}", response);
    //
    //     response
    // }

    fn modify_trade() {
        todo!()
    }
    fn close_trade() {
        todo!()
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
    pub fn get_existing_trades() -> Result<String, Box<dyn std::error::Error>> {
        let bridge = Self::init();
        let data = "TRADE;GET_OPEN_TRADES";
        let response = bridge.sockets.request(data, 0).receive();

        Ok(response)
    }
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
    //
    pub fn mt5_date_from(date: chrono::DateTime<Utc>) -> String {
        let date = std::time::Instant::now();
        let date = format!("{:#?}", date);
        println!("Here's the date: {date}");
        todo!()
        // let date: String = date.into();
        // String::from(date)
    }
    // }
    // pub fn get_existing_trades() -> Result<String, Box<dyn std::error::Error>> {
    //     let data = "TRADE;GET_OPEN_TRADES";
    //     let sockets = ConnectionSockets::init_and_connect()?;
    //     let response = sockets.request(data, 0)?.receive()?;
    //
    //     Ok(response)
}

//SYMBOLS
impl Mt5Bridge {
    pub fn get_symbols() -> Vec<Symbol> {
        let data = "DATA;GET_SYMBOLS";
        let bridge = Self::init();
        let response = bridge.sockets.request(data, 1).receive();

        // println!("Symbols received from mt5: {:#?}", response);

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
        let mut data = self.sanitize_mt5_response(&data);
        let data = data.remove("symbol_data").unwrap();
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

    }
    // Replace single quotations with double for parsing with serde_json
    // Remove the action key term located in almost every request.
    fn sanitize_mt5_response(&self, response: &str) -> serde_json::Map<String, Value> {
        let response = response.replace("'", "\"");
        let response = serde_json::from_str(&response).expect(&format!(
            "Unable to parse string to Map<String, Value>\n Received String: \n {}",
            response
        ));
        response
        // let request = data.get("action").unwrap();
        // let request = serde_json::to_string(request).unwrap().replace("\"", "");
        // data.remove("action");
        // let data = serde_json::to_string(&data).expect(&format!(
        //     "Unable to parse serde_json Map to String. \n Received Map: \n {:#?}",
        //     data
        // ));
        // panic!("The data string: {:#?}",  data);
        // (request, data)
    }
}
