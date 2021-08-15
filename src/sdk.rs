use std::sync::Arc;

use async_trait::async_trait;
use rusoto_core::Region;
use rusoto_credential::{AwsCredentials, CredentialsError, ProvideAwsCredentials, StaticProvider};
use rusoto_s3::S3Client;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};

#[derive(Debug, Clone)]
pub struct CustomStsProvider {
    pub user_creds: StaticProvider,
    pub assume_role_arn: String,
    pub external_id: Option<String>,
    pub region: Region,
}

impl CustomStsProvider {
    pub fn new(
        access_key_id: String,
        secret_access_key: String,
        assume_role_arn: String,
        external_id: Option<String>,
        region: Region,
    ) -> CustomStsProvider {
        CustomStsProvider {
            user_creds: StaticProvider::new_minimal(access_key_id, secret_access_key),
            assume_role_arn,
            external_id,
            region,
        }
    }
}

#[async_trait]
impl ProvideAwsCredentials for CustomStsProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let http_client = rusoto_core::HttpClient::new().unwrap();
        let arced_client = Arc::new(http_client);
        let client =
            StsClient::new_with(arced_client, self.user_creds.clone(), self.region.clone());

        let request: AssumeRoleRequest = AssumeRoleRequest {
            external_id: match &self.external_id {
                Some(ex) => Some(ex.clone()),
                None => None,
            },
            role_arn: self.assume_role_arn.clone(),
            role_session_name: format!("ipnotifier-session-{}", crate::utils::get_time_in_millis()),
            ..Default::default()
        };

        let response = client
            .assume_role(request)
            .await
            .or_else(|err| Err(CredentialsError::new(err.to_string())))?;
        let creds = response
            .credentials
            .ok_or_else(|| CredentialsError::new("Did not find credentials in response"))?;

        Ok(AwsCredentials::new(
            creds.access_key_id,
            creds.secret_access_key,
            Some(creds.session_token),
            Some(crate::utils::parse_into_utc(creds.expiration)),
        ))
    }
}

pub fn get_s3_client<P>(credential_provider: P, region: Region) -> S3Client
where
    P: ProvideAwsCredentials + Sync + Send + 'static,
{
    let http_client = rusoto_core::HttpClient::new().unwrap();
    let arced_client = Arc::new(http_client);

    S3Client::new_with(arced_client, credential_provider, region)
}
