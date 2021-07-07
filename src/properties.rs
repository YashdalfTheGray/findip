use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone)]
pub enum Notifier {
    File {
        overwrite: bool,
        file_path: Box<Path>,
    },
    S3 {
        assume_role_arn: String,
        region: String,
        bucket_name: String,
    },
    RestApi {
        url: String,
        method: String,
        body: HashMap<String, String>,
        headers: HashMap<String, String>,
    },
    Stdout,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct FileProperties {
    overwrite: bool,
    file_path: Box<Path>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct S3Properties {
    assume_role_arn: String,
    region: String,
    bucket_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct RestProperties {
    endpoint: url::Url,
    #[serde(default)]
    headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct ConfigFile<T> {
    cron: String,
    notify_on_change_only: bool,
    notifiers: Vec<T>,
}
