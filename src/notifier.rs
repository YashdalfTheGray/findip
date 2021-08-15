use std::{collections::HashMap, fs, io::Write, net::IpAddr};

use chrono::Utc;
use log::{debug, error};
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, StreamingBody, S3};

use crate::{
    errors::{ErrorReason, IpError},
    sdk::{get_s3_client, CustomStsProvider},
};

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

impl IpNotifier for S3Notifier {
    fn notify_success(&self, ip: IpAddr) {
        let parsed_region = self.region.parse::<Region>().unwrap_or(Region::UsWest2);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let credentials_provider = CustomStsProvider::new(
            self.access_key_id.clone(),
            self.secret_access_key.clone(),
            self.assume_role_arn.clone(),
            None,
            parsed_region.clone(),
        );
        let client = get_s3_client(credentials_provider, parsed_region.clone());
        let key = format!(
            "{}-ipnotification.txt",
            Utc::now().format("%Y-%m-%d-%H").to_string()
        );

        let put_object_request = PutObjectRequest {
            bucket: self.bucket_name.clone(),
            body: Some(StreamingBody::from(ip.to_string().as_bytes().to_vec())),
            key,
            ..Default::default()
        };

        let future_response = client.put_object(put_object_request);
        let response = runtime.block_on(future_response);

        match response {
            Ok(output) => {
                debug!("IP written to S3 successfully. Output follows.");
                debug!("{:#?}", output);
            }
            Err(err) => IpNotifier::notify_error(
                self,
                IpError::new(ErrorReason::S3WriteFailed(err.to_string())),
            ),
        }
    }

    fn notify_error(&self, err: IpError) {}
}

pub struct RestNotifier {
    url: String,
    method: String,
    body: HashMap<String, String>,
    headers: HashMap<String, String>,
}
