use serde_json::{json, Value};
use icrc_ledger_types::icrc::generic_value::ICRC3Value;

pub fn icrc3_to_json(value: &ICRC3Value) -> Value {
    match value {
        ICRC3Value::Text(text) => json!(text),
        ICRC3Value::Nat(num) => {
            let num_str = num.to_string().replace("_", "");  // Remove underscores
            if num_str.len() > 16 {  // Likely a nanosecond timestamp
                let seconds = num_str.chars().take(num_str.len() - 9).collect::<String>();
                if let Ok(n) = seconds.parse::<u64>() {
                    json!(n)
                } else {
                    json!(num_str)
                }
            } else if let Ok(n) = num_str.parse::<u64>() {
                json!(n)
            } else {
                json!(num_str)
            }
        },
        ICRC3Value::Int(num) => {
            let num_str = num.to_string().replace("_", "");  // Remove underscores
            if let Ok(n) = num_str.parse::<i64>() {
                json!(n)
            } else {
                json!(num_str)
            }
        },
        ICRC3Value::Blob(bytes) => {
            if bytes.is_empty() {
                json!(null)
            } else {
                json!(hex::encode(bytes))
            }
        },
        ICRC3Value::Map(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                json_map.insert(k.clone(), icrc3_to_json(v));
            }
            Value::Object(json_map)
        },
        ICRC3Value::Array(arr) => {
            json!(arr.iter().map(icrc3_to_json).collect::<Vec<_>>())
        }
   }
}

// Helper function to get JSON string without the "opt" wrapper
pub fn get_json_string(value: &ICRC3Value) -> String {
    serde_json::to_string_pretty(&icrc3_to_json(value))
        .unwrap_or_else(|_| "Error converting to JSON".to_string())
}

pub fn get_json_string_from_vec(vec: &Vec<ICRC3Value>) -> String {
    vec.iter().map(get_json_string).collect::<Vec<_>>().join(", ")
}