use std::{iter::Map, path::Path};

#[derive(Debug, Clone)]
pub struct FileProperties {
    overwrite: bool,
    file_path: Box<Path>,
}

#[derive(Debug, Clone)]
pub struct S3Properties {
    assume_role_arn: String,
    region: String,
    bucket_name: String,
}

#[derive(Debug, Clone)]
pub struct RestProperties {
    endpoint: url::Url,
    headers: Map<String, String>,
}
