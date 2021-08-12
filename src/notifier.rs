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

impl IpNotifier for FileNotifier {
    fn notify_success(&self, ip: IpAddr) {
        if self.overwrite {
            match fs::write(ip.to_string(), self.file_path.clone()) {
                Ok(()) => (),
                Err(_) => IpNotifier::notify_error(
                    self,
                    IpError::new(ErrorReason::FileWriteFailed(self.file_path.clone())),
                ),
            }
        } else {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.file_path.clone())
                .expect("failed to open file");

            match file.write(&ip.to_string().into_bytes()) {
                Ok(_) => (),
                Err(_) => IpNotifier::notify_error(
                    self,
                    IpError::new(ErrorReason::FileWriteFailed(self.file_path.clone())),
                ),
            }
        }
    }
    fn notify_error(&self, err: IpError) {}
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
