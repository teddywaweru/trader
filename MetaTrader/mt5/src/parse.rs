use crate::{Indicator, InstantRates, OpenTrade, Order};
use serde_json::{Map, Value};
fn parse_trade(data: String) -> Result<Order, serde_json::Error> {
    let mut data: Map<String, Value> = serde_json::from_str(&data)?;
    data.remove("action");
    let data = serde_json::to_string(&data)?;
    let order: Order = serde_json::from_str(&data)?;
    Ok(order)
}
fn parse_price_data(data: String) -> Result<InstantRates, serde_json::Error> {
    let mut instant_rates: Map<String, Value> = serde_json::from_str(&data)?;
    instant_rates.remove("action");
    let data = serde_json::to_string(&instant_rates)?;
    let instant_rates: InstantRates = serde_json::from_str(&data)?;
    Ok(instant_rates)
}
pub fn parse_indicator_data(data: String) -> Result<Indicator, serde_json::Error> {
    let mut indicator_data: Map<String, Value> = serde_json::from_str(&data)?;
    indicator_data.remove("action");
    let indicator_data: String = serde_json::to_string(&indicator_data)?;
    // let indicator_data: Indicator = serde_json::from_str(&indicator_data)?;
    // Ok(indicator_data)
    todo!()
}

#[cfg(test)]
mod parse_data_tests {
    use super::*;
    use std::fs::File;
    use std::io::{self, BufRead};
    #[test]
    fn parse_open_trades_test() {
        let file = File::open("test_data/current_trades.json").unwrap();
        let reader = io::BufReader::new(file);
        let data = reader
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .join("");
        // for lines in reader.lines() {
        // let line = lines.unwrap();
        // let open_trades = parse_open_trades(data).unwrap();
        // println!("The lines then {:#?}", open_trades);
        // }
        unimplemented!();
    }
}
