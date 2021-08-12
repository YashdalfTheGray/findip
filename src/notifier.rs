use std::{collections::HashMap, fs, io::Write, net::IpAddr};

use crate::errors::{ErrorReason, IpError};

pub trait IpNotifier {
    fn notify_success(&self, ip: IpAddr);
    fn notify_error(&self, err: IpError);
}

pub struct FileNotifier {
    overwrite: bool,
    file_path: String,
}

impl FileNotifier {
    pub fn new(file_path: String, overwrite: bool) -> FileNotifier {
        FileNotifier {
            file_path,
            overwrite,
        }
    }
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
