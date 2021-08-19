use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::prelude::*;

pub fn get_time_in_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

pub fn parse_into_utc(expiration_str: String) -> DateTime<Utc> {
    DateTime::from_utc(
        DateTime::parse_from_rfc3339(&expiration_str)
            .unwrap()
            .naive_utc(),
        Utc,
    )
}

pub fn replace_tokens(original: String, token_value_map: HashMap<String, String>) -> String {
    let result = original.clone();

    token_value_map
        .iter()
        .fold(result, |acc, (key, value)| acc.replace(key, value))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_replace_tokens_with_no_tokens() {
        let original = "test string".to_string();
        let token_value_map = HashMap::<String, String>::new();

        let result = replace_tokens(original, token_value_map);
        assert_eq!(result, "test string");
    }

    #[test]
    fn test_replace_tokens_with_one_token() {
        let original = "test string with {{TOKEN}}".to_string();
        let mut token_value_map = HashMap::new();
        token_value_map.insert("{{TOKEN}}".to_string(), "token value!".to_string());

        let result = replace_tokens(original, token_value_map);
        assert_eq!(result, "test string with token value!");
    }
}
