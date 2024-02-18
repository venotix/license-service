use tonic::transport::Channel;

use crate::licensing::LicenseItem;
use crate::networking::license::license_client::LicenseClient;
use crate::networking::license::LicenseRequest;

pub struct LicenseClientBootstrap {
    pub client: LicenseClient<Channel>,
    pub port: u16,
}

impl LicenseClientBootstrap {
    pub async fn new(port: u16) -> Option<Self> {
        let client = LicenseClient::connect(format!("http://[::1]:{}", &port)).await;

        match client {
            Ok(client) => {
                Some(Self {
                    client,
                    port
                })
            }
            Err(_) => {
                None
            }
        }
    }

    pub async fn request_license(&mut self, license: String) -> Option<LicenseItem> {
        match self.client.request_license(LicenseRequest {
            license
        }).await {
            Ok(response) => {
                let response = response.into_inner();

                Some(LicenseItem {
                    license: response.license,
                    attributes: response.attributes
                })
            }
            Err(_) => {
                None
            }
        }
    }
}