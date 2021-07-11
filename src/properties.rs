use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt, fs::read_to_string, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(tag = "notifierType", content = "properties")]
pub enum Notifier {
    #[serde(rename_all(deserialize = "camelCase"))]
    File {
        overwrite: bool,
        file_path: Box<Path>,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    S3 {
        assume_role_arn: String,
        region: String,
        bucket_name: String,
    },
    #[serde(rename_all(deserialize = "camelCase"))]
    RestApi {
        url: String,
        method: String,
        body: HashMap<String, String>,
        headers: HashMap<String, String>,
    },
    Stdout,
}

impl fmt::Display for Notifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ConfigFile {
    cron: String,
    notify_on_change_only: bool,
    notifiers: Vec<Notifier>,
}

pub fn load_config_from_file(file_path: String) -> Result<ConfigFile, Box<dyn Error + 'static>> {
    let contents = read_to_string(file_path)?;
    let config_file: ConfigFile = serde_yaml::from_str(&contents)?;
    Ok(config_file)
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fmt;

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
        match &config_file.notifiers[0] {
            Notifier::S3 {
                assume_role_arn,
                region,
                bucket_name,
            } => {
                assert_eq!(assume_role_arn, "roleArn");
                assert_eq!(region, "us-west-2");
                assert_eq!(bucket_name, "bucketName");
                Ok(())
            }
            _ => Err(Box::new(UnexpectedNotifierError {
                expected: Notifier::S3 {
                    assume_role_arn: "".to_owned(),
                    region: "".to_owned(),
                    bucket_name: "".to_owned(),
                },
            })),
        }
    }
}
