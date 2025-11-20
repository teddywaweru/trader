use crate::mt5_bridge::Mt5Bridge;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_number: i32,
    pub name: String,
    //company
    //server 
    //mode
    pub current_time: String,
    pub currency: String,
    pub current_equity: f32,
    pub current_balance: f32,
    pub free_margin: f32,
    pub current_profit: f32,
    pub leverage: u32,

}
// "{'_action': 'GET_ACCOUNT_INFO', 'account_number':61189795, '_data': [{'current_time': '2024.04.19
// 08:56', 'name':'Teddy Waweru Njuguna', 'balance':40862.08000000, 'equity':41827.15000000, 'profit':965.07000000, 'margin_free':41003.31000000,
//  'leverage' :400}]}"

impl Default for Account {
    fn default() -> Self {
        let account_number = 65;
        let name = "njuwate".to_string();
        let current_time = "2024".to_string();
        let current_balance = 3434.233;
        let current_equity = 3234.55;
        let leverage = 500;
        let currency = "USD".to_string();
        let free_margin = 3543.34;
        let current_profit = 333.2;
        Self {
            account_number,
            name,
            current_time,
            current_balance,
            current_equity,
            leverage,
            currency,
            free_margin,
            current_profit,
        }
    }
}
impl Account {
    pub(crate) fn get_balance() -> f32 {
        todo!()
    }
    pub fn get_account_info() -> Self {

        let data = Mt5Bridge::get_account_info();

            data

    }
    pub fn parse_mt5_response(data: &str) -> Account {

        let account = match serde_json::from_str(&data) {
            Ok(account) => account,
            Err(e) => {
                println!(
                    "Unable to parse Account Data collected from MT5:\n
                       String Received: {}, \n Error: {}",
                    &data, e
                );
                println!("Sharing default Account information for pseudo use.");
                let account = Account::default();
                account
            }
        };
        account
    }
}

impl From<&str> for Account {
    fn from(account: &str) -> Self {
        Account::default()
    }
}
