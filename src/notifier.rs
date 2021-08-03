use std::{collections::HashMap, fs::File, net::IpAddr};

use crate::errors::IpError;

pub trait IpNotifier {
    fn notify_success(&self, ip: IpAddr);
    fn notify_error(&self, err: IpError);
}

pub struct FileNotifier {
    overwrite: bool,
    file: Option<File>,
}

pub struct S3Notifier {
    assume_role_arn: String,
    region: String,
    bucket_name: String,
}

pub struct RestNotifier {
    url: String,
    method: String,
    body: HashMap<String, String>,
    headers: HashMap<String, String>,
}
