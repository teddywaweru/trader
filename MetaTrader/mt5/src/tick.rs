use crate::{mt5_bridge::Mt5Bridge, serde_timeframe, Symbol, Timeframe, OHLC};
use chrono::{prelude::*, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTickData {
    #[serde(with = "serde_timeframe")]
    timeframe: Timeframe,
    pub ticks: Vec<OHLC>,
}

impl HistoricalTickData {
    pub fn get(duration: u32) -> String {
        let request = "HIST;EURUSD;1440;2023.08.01 00:00:00;2023.08.4 00:00:00";
        let flag = 0;
        let response = "".to_owned();
        let historical_data: Vec<OHLC>;
        let x = HistoricalTickData::from_mt5_response(&response);
        // let x = Mt5Bridge::get_historical_tick_data()
        format!("stuff, {:#?}", x)
    }
    pub fn from_mt5_response(data: &str) -> Self {
        let data =
            serde_json::from_str(&data).expect(&format!("Unable to parse string: \n {data}"));
        data
    }
}
// impl Deserialize
// impl From<String> for HistoricalTickData {
//     fn from(value: String) -> Self {
//         // Optional Implementations:
//         // 1. Parse each tick data individually then add them to a Vec
//         // 2. nvestigate How to implement Vec parsing for serde_json
//         println!("{value}");
//         // let parsed_data: serde_json::Map<String, serde_json::Value> =
//         // serde_json::from_str(&value).unwrap();
//         // let data = parsed_data.get("_data").unwrap().to_owned();
//         let val: HistoricalTickData = serde_json::from_str(&value).unwrap();
//         val
//     }
// }

pub struct HistoricalTickDataRequest<'a> {
    symbol: &'a Symbol,
    timeframe: Timeframe,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

// impl <'a>Default for HistoricalTickDataRequest<'a> {
//     fn default() -> Self<'_> {
//         Self {
//             symbol: &Symbol::default(),
//             timeframe: Timeframe::default(),
//             start_date: Utc::now(),
//             end_date: Utc::now(),
//         }
//     }
// }
impl<'a> HistoricalTickDataRequest<'a> {
    pub fn new(
        symbol: &'a Symbol,
        timeframe: Timeframe,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Self {
        Self {
            symbol,
            timeframe,
            start_date,
            end_date,
        }
    }
    pub fn to_mt5_request(self) -> String {
        // 2.2) DATA|SYMBOL|TIMEFRAME|START_DATETIME|END_DATETIME
        // assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09"
        // NOTE: datetime has format: D'2015.01.01 00:00'
        let start_date = self.start_date.format("D'%Y.%m.%d %H:%M'");
        let data = format!(
            "TRADE;GET_OPEN_TRADES;{};{},{},{}",
            self.symbol.name, self.timeframe as u8, self.start_date, self.end_date
        );
        println!("The HistoricalTickDataRequest data: {start_date}");
        data
    }
}
