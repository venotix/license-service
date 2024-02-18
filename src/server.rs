use std::net::ToSocketAddrs;
use std::sync::Arc;

use tonic::transport::Server;

use crate::licensing::LicenseRepository;
use crate::networking::license::license_server::LicenseServer;
use crate::networking::NetworkLicenseServer;

pub struct LicenseServerBootstrap<T: LicenseRepository> {
    pub repository: T,
    pub port: u16
}

impl<T> LicenseServerBootstrap<T> where T: LicenseRepository + Sync + Send + Clone + 'static {
    pub async fn run(&self) {
        let repository = Arc::new(self.repository.clone());

        let networking_server = NetworkLicenseServer {
            repository: repository
        };

        println!("License server listening on [::1]:{}", &self.port);

        Server::builder()
            .add_service(LicenseServer::new(networking_server))
            .serve(format!("[::1]:{}", &self.port).to_socket_addrs().unwrap().next().unwrap())
            .await
            .unwrap();
    }
}