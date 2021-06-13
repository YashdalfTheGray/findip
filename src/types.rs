use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum NotifierStrategy {
    File,
    Stdout,
    Endpoint,
    S3,
}
