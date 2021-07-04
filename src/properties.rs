use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileProperties {
    overwrite: bool,
    file_path: Box<Path>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Properties {
    assume_role_arn: String,
    region: String,
    bucket_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestProperties {
    endpoint: url::Url,
    headers: HashMap<String, String>,
}
