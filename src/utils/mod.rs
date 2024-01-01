use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

pub fn get_env() -> HashMap<String, String> {
    dotenv().ok();
    let mut temp = HashMap::new();
    for (key, value) in env::vars() {
        temp.insert(key, value);
    }
    temp
}
