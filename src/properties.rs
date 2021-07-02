use std::path::Path;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileProperties {
    overwrite: bool,
    file_path: Box<Path>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct S3Properties {
    assume_role_arn: String,
    region: String,
    bucket_name: String,
}
