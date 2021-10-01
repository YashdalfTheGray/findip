use std::{
    collections::HashMap,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::prelude::*;
use clap::ArgMatches;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbosityLevel {
    Standard,
    Verbose = 1,
    PrettyNoisy,
    LiterallyEverything,
}

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

pub fn generate_error_file_path(log_file_path: String) -> String {
    let error_file_path = Path::new(&log_file_path);
    let file_stem = error_file_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let extension = error_file_path.extension().unwrap().to_str().unwrap();

    error_file_path
        .with_file_name(file_stem + ".errors." + extension)
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_verbosity(matches: &ArgMatches) -> VerbosityLevel {
    match matches.occurrences_of("verbose") {
        0 => VerbosityLevel::Standard,
        1 => VerbosityLevel::Verbose,
        2 => VerbosityLevel::PrettyNoisy,
        3..=u64::MAX => VerbosityLevel::LiterallyEverything,
    }
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

    #[test]
    fn test_replace_tokens_with_no_tokens_in_input() {
        let original = "test string without tokens".to_string();
        let mut token_value_map = HashMap::new();
        token_value_map.insert("{{TOKEN}}".to_string(), "token value!".to_string());

        let result = replace_tokens(original, token_value_map);
        assert_eq!(result, "test string without tokens");
    }

    #[test]
    fn test_replace_tokens_with_multiple_tokens() {
        let original = "test string with {{TOKEN1}} and {{TOKEN2}}".to_string();
        let mut token_value_map = HashMap::new();
        token_value_map.insert("{{TOKEN1}}".to_string(), "token value 1".to_string());
        token_value_map.insert("{{TOKEN2}}".to_string(), "token value 2!".to_string());

        let result = replace_tokens(original, token_value_map);
        assert_eq!(result, "test string with token value 1 and token value 2!");
    }

    #[test]
    fn test_generate_error_file_path_with_just_file_name() {
        let log_file_path = "output.log".to_string();
        let error_file_path = generate_error_file_path(log_file_path);
        assert_eq!(error_file_path, "output.errors.log".to_string());
    }

    #[test]
    fn test_generate_error_file_path_with_relative_path() {
        let log_file_path = "var/logs/output.log".to_string();
        let error_file_path = generate_error_file_path(log_file_path);
        assert_eq!(error_file_path, "var/logs/output.errors.log".to_string());
    }

    #[test]
    fn test_generate_error_file_path_with_absolute_path() {
        let log_file_path = "/tmp/logs/output.log".to_string();
        let error_file_path = generate_error_file_path(log_file_path);
        assert_eq!(error_file_path, "/tmp/logs/output.errors.log".to_string());
    }

    #[test]
    fn test_generate_error_file_path_with_dots() {
        let log_file_path = "./logs/output.log".to_string();
        let error_file_path = generate_error_file_path(log_file_path);
        assert_eq!(error_file_path, "./logs/output.errors.log".to_string());
    }
}
