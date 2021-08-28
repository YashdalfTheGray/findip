use aws_arn::ARN;
use http::HeaderMap;
use log::LevelFilter;
use reqwest::Method;
use rusoto_core::Region;
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, error::Error, fmt, fs::read_to_string, str::FromStr};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(tag = "notifierType", content = "properties")]
pub enum Notifier {
    #[serde(rename_all(deserialize = "camelCase"))]
    File {
        overwrite: bool,
        file_path: String,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    S3 {
        access_key_id: String,
        secret_access_key: String,
        #[serde(deserialize_with = "deserialize_arn_from_string")]
        assume_role_arn: ARN,
        region: Region,
        bucket_name: String,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    RestApi {
        url: String,
        #[serde(with = "http_serde::method")]
        method: Method,
        body: HashMap<String, String>,
        #[serde(with = "http_serde::header_map")]
        headers: HeaderMap,
    },
    Stdout,
}

impl fmt::Display for Notifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LoggingConfig {
    #[serde(default = "get_default_log_path")]
    pub log_file: String,
    #[serde(default = "get_default_logging_level")]
    pub log_level: LevelFilter,
    #[serde(default = "get_default_log_decoration")]
    pub decorate: bool,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ConfigFile {
    cron: String,
    #[serde(default = "get_default_services")]
    #[validate(length(min = 1))]
    services: Vec<String>,
    notify_on_change_only: bool,
    #[validate(length(equal = 1))]
    notifiers: Vec<Notifier>,
    #[serde(default = "get_default_logging_config")]
    pub logging_config: LoggingConfig,
}

pub fn get_default_services() -> Vec<String> {
    vec![
        "https://api.ipify.org/".to_string(),
        "https://diagnostic.opendns.com/myip".to_string(),
    ]
}

pub fn get_default_logging_config() -> LoggingConfig {
    let log_path = "/tmp/ip_notifier.log";

    LoggingConfig {
        log_file: log_path.to_string(),
        log_level: LevelFilter::Info,
        decorate: true,
    }
}

pub fn get_default_log_path() -> String {
    get_default_logging_config().log_file
}

pub fn get_default_logging_level() -> LevelFilter {
    get_default_logging_config().log_level
}

pub fn get_default_log_decoration() -> bool {
    get_default_logging_config().decorate
}

pub fn load_config_from_file(file_path: String) -> Result<ConfigFile, Box<dyn Error + 'static>> {
    let contents = read_to_string(file_path)?;
    let config_file: ConfigFile = serde_yaml::from_str(&contents)?;
    Ok(config_file)
}

fn deserialize_arn_from_string<'de, D>(deserializer: D) -> Result<ARN, D::Error>
where
    D: Deserializer<'de>,
{
    let input = String::deserialize(deserializer)?;
    ARN::from_str(&input).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fmt;
    use std::str::FromStr;

    use super::*;

    #[derive(Debug, Clone)]
    struct UnexpectedNotifierError {
        expected: Notifier,
    }

    impl fmt::Display for UnexpectedNotifierError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Notifier did not match what was expected, expected was {}",
                self.expected
            )
        }
    }

    impl Error for UnexpectedNotifierError {}

    #[test]
    fn test_stdout_notifier_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/stdout.yml".to_string())?;
        assert_eq!(config_file.notifiers[0], Notifier::Stdout);
        Ok(())
    }

    #[test]
    fn test_s3_notifier_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/s3.yml".to_string())?;

        if let Notifier::S3 {
            access_key_id,
            secret_access_key,
            assume_role_arn,
            region,
            bucket_name,
        } = &config_file.notifiers[0]
        {
            assert_eq!(access_key_id, "something");
            assert_eq!(secret_access_key, "shhh");
            assert_eq!(
                assume_role_arn.to_string(),
                "arn:aws:iam::123456789012:role/namespace/assume-role"
            );
            assert_eq!(*region, Region::UsWest2);
            assert_eq!(bucket_name, "bucketName");
            Ok(())
        } else {
            Err(Box::new(UnexpectedNotifierError {
                expected: Notifier::S3 {
                    access_key_id: "".to_owned(),
                    secret_access_key: "".to_owned(),
                    assume_role_arn: ARN::from_str(
                        "arn:aws:iam::123456789012:role/namespace/assume-role",
                    )
                    .unwrap(),
                    region: Region::UsWest2,
                    bucket_name: "".to_owned(),
                },
            }))
        }
    }

    #[test]
    fn test_rest_notifier_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/rest.yml".to_string())?;

        if let Notifier::RestApi {
            url,
            method,
            body,
            headers,
        } = &config_file.notifiers[0]
        {
            assert_eq!(url, "https://something.com/some/api");
            assert_eq!(method, Method::POST);
            assert_eq!(*body.get("ip").unwrap(), "{{TOKEN_IP_ADDRESS}}".to_owned());
            assert_eq!(
                *headers.get("Authorization").unwrap(),
                "Bearer mysecrettoken".to_owned()
            );
            assert_eq!(
                *headers.get("Content-Type").unwrap(),
                "application/json".to_owned()
            );
            Ok(())
        } else {
            Err(Box::new(UnexpectedNotifierError {
                expected: Notifier::RestApi {
                    url: "".to_owned(),
                    method: Method::GET,
                    body: HashMap::new(),
                    headers: HeaderMap::new(),
                },
            }))
        }
    }

    #[test]
    fn test_rest_notifier_body_serialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/rest.yml".to_string())?;

        if let Notifier::RestApi {
            url: _,
            method: _,
            body,
            headers: _,
        } = &config_file.notifiers[0]
        {
            assert_eq!(
                serde_json::to_string(&*body).unwrap(),
                "{\"ip\":\"{{TOKEN_IP_ADDRESS}}\"}".to_owned()
            );
            Ok(())
        } else {
            Err(Box::new(UnexpectedNotifierError {
                expected: Notifier::RestApi {
                    url: "".to_owned(),
                    method: Method::GET,
                    body: HashMap::new(),
                    headers: HeaderMap::new(),
                },
            }))
        }
    }

    #[test]
    fn test_file_notifier_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/textfile.yml".to_string())?;

        if let Notifier::File {
            overwrite,
            file_path,
        } = &config_file.notifiers[0]
        {
            assert_eq!(*overwrite, false);
            assert_eq!(*file_path, "testfile.log".to_owned());
            Ok(())
        } else {
            Err(Box::new(UnexpectedNotifierError {
                expected: Notifier::File {
                    overwrite: false,
                    file_path: "".to_owned(),
                },
            }))
        }
    }

    #[test]
    fn test_missing_services_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/textfile.yml".to_string())?;

        assert_eq!(config_file.services, get_default_services());
        Ok(())
    }

    #[test]
    fn test_included_services_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/custom_services.yml".to_string())?;

        assert_eq!(
            config_file.services,
            vec!["https://ipinfo.io/ip".to_string()]
        );
        Ok(())
    }

    #[test]
    fn test_logging_config_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/full_logging_config.yml".to_string())?;

        assert_eq!(
            config_file.logging_config.log_file,
            "./var/log/notifier.log".to_owned()
        );
        assert_eq!(config_file.logging_config.log_level, LevelFilter::Warn);
        assert_eq!(config_file.logging_config.decorate, false);
        Ok(())
    }

    #[test]
    fn test_logging_config_deserialization_with_defaults() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = load_config_from_file("testfiles/stdout.yml".to_string())?;

        assert_eq!(
            config_file.logging_config.log_file,
            "/tmp/ip_notifier.log".to_owned()
        );
        assert_eq!(config_file.logging_config.log_level, LevelFilter::Info);
        assert_eq!(config_file.logging_config.decorate, true);
        Ok(())
    }

    #[test]
    fn test_logging_config_deserialization_with_partial_defaults(
    ) -> Result<(), Box<dyn Error + 'static>> {
        let config_file =
            load_config_from_file("testfiles/partial_logging_config.yml".to_string())?;

        assert_eq!(
            config_file.logging_config.log_file,
            "./var/log/notifier.log".to_owned()
        );
        assert_eq!(config_file.logging_config.log_level, LevelFilter::Trace);
        assert_eq!(config_file.logging_config.decorate, true);
        Ok(())
    }
}
