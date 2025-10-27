use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mt5Error {
    error_retcode: u32,
    error_message: String,
}
impl std::fmt::Display for Mt5Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for Mt5Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }

    // fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}
impl Mt5Error {
    fn from_mt5_response(data: String) -> Self {
        todo!()
    }
    pub fn check_if_mt5_error(data: &str) {
        use serde_json::{Map, Value};
        let x: Map<String, Value> = serde_json::from_str(data).unwrap();
        println!(
            "Value of action: {}, value of response_value: {}",
            x["action"], x["response_value"]
        );
        match serde_json::from_str::<Map<String, Value>>(data) {
            Ok(_) => {}
            Err(error) => {
                let x: Self = serde_json::from_str(data).unwrap();
                // if x["action"] == ""

                panic!("Message from ");
            }
        }
    }
    pub fn new<T>(context: &str, error: T) -> Self
    where
        T: std::error::Error,
    {
        Self {
            error_retcode: 000000,
            error_message: format!("{}\n {}", context, error),
        }
    }
}
