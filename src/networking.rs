use std::collections::HashMap;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::licensing::{LicenseItem, LicenseRepository};
use crate::networking::license::license_server::License;
use crate::networking::license::{LicenseRequest, LicenseResponse};
use crate::licensing;

pub mod license {
    tonic::include_proto!("license");
}

pub struct NetworkLicenseServer<T: LicenseRepository> {
    pub repository: Arc<T>,
}

#[tonic::async_trait]
impl<T: Sync + Send + LicenseRepository + 'static> License for NetworkLicenseServer<T> {
    async fn request_license(&self, request: Request<LicenseRequest>) -> Result<Response<LicenseResponse>, Status> {
        let request = request.into_inner();
        let license = &self.repository.get_license(request.license);

        match license {
            None => {
                return Err(Status::not_found("License not found"));
            }
            Some(item) => {
                return Ok(Response::new(LicenseResponse {
                    license: item.license.clone(),
                    attributes: item.attributes.clone(),
                }));
            }
        }
    }
}