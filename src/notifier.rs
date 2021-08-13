use std::{collections::HashMap, fs, io::Write, net::IpAddr};

use log::{debug, error};

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
                Ok(()) => debug!("IP written to file successfully."),
                Err(_) => IpNotifier::notify_error(
                    self,
                    IpError::new(ErrorReason::FileWriteFailed(self.file_path.clone())),
                ),
            }
        } else {
            match fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.file_path.clone())
            {
                Ok(mut file) => match file.write(&ip.to_string().into_bytes()) {
                    Ok(_) => debug!("IP written to file successfully."),
                    Err(_) => IpNotifier::notify_error(
                        self,
                        IpError::new(ErrorReason::FileWriteFailed(self.file_path.clone())),
                    ),
                },
                Err(_) => IpNotifier::notify_error(
                    self,
                    IpError::new(ErrorReason::FileOpenFailed(self.file_path.clone())),
                ),
            }
        }
    }
    fn notify_error(&self, err: IpError) {
        error!("{}", err)
    }
}

pub struct S3Notifier {
    access_key_id: String,
    secret_access_key: String,
    assume_role_arn: String,
    region: String,
    bucket_name: String,
}

impl S3Notifier {
    pub fn new(
        access_key_id: String,
        secret_access_key: String,
        assume_role_arn: String,
        region: String,
        bucket_name: String,
    ) -> S3Notifier {
        S3Notifier {
            access_key_id,
            secret_access_key,
            assume_role_arn,
            region,
            bucket_name,
        }
    }
}

pub struct RestNotifier {
    url: String,
    method: String,
    body: HashMap<String, String>,
    headers: HashMap<String, String>,
}
