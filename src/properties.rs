use serde::Deserialize;
use std::{collections::HashMap, path::Path};

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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ConfigFile {
    cron: String,
    notify_on_change_only: bool,
    notifiers: Vec<Notifier>,
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::read_to_string;

    use super::*;

    fn get_yaml_from_file(file_path: String) -> Result<ConfigFile, Box<dyn Error + 'static>> {
        let contents = read_to_string(file_path)?;
        let config_file: ConfigFile = serde_yaml::from_str(&contents)?;
        Ok(config_file)
    }

    #[test]
    fn test_stdout_notifier_deserialization() -> Result<(), Box<dyn Error + 'static>> {
        let config_file = get_yaml_from_file("testfiles/stdout.yml".to_string())?;
        assert_eq!(config_file.notifiers[0], Notifier::Stdout);
        Ok(())
    }
}
