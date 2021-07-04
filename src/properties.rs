use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileProperties {
    overwrite: bool,
    file_path: Box<Path>,
}

#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Properties {
    assume_role_arn: String,
    region: String,
    bucket_name: String,
}

#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestProperties {
    endpoint: url::Url,
    #[serde(default)]
    headers: HashMap<String, String>,
}
