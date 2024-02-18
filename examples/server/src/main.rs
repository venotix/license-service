use std::collections::HashMap;

use vlks::licensing::LicenseItem;
use vlks::licensing::LicenseRepository;
use vlks::server::LicenseServerBootstrap;

#[tokio::main]
async fn main() {
    let repository = MyLicenseRepository {
        licenses: vec![
            LicenseItem {
                license: "c97283f4-06ee-46c1-8e39-d337483f4eb1".to_string(),
                attributes: HashMap::from([
                    ("Company".to_string(), "Sample1".to_string()),
                ])
            },
            LicenseItem {
                license: "c97283f4-06ee-46c1-8e39-d337483f4eb2".to_string(),
                attributes: HashMap::from([
                    ("Company".to_string(), "Sample2".to_string()),
                ])
            },
            LicenseItem {
                license: "c97283f4-06ee-46c1-8e39-d337483f4eb3".to_string(),
                attributes: HashMap::from([
                    ("Company".to_string(), "Sample3".to_string()),
                ])
            }
        ]
    };

    let license_server = LicenseServerBootstrap::<MyLicenseRepository> {
        repository: repository,
        port: 22623
    };

    license_server.run().await;
}

#[derive(Debug, Clone)]
struct MyLicenseRepository {
    licenses: Vec<LicenseItem>,
}

impl LicenseRepository for MyLicenseRepository {
    fn add_license(&mut self, license_item: LicenseItem) {
        self.licenses.push(license_item);
    }
    fn get_license(&self, license: String) -> Option<LicenseItem> {
        for item in &self.licenses {
            if item.clone().license == license {
                return Some(item.clone());
            }
        }
        None
    }
    fn get_licenses(self) -> Vec<LicenseItem> {
        self.licenses
    }
    fn delete_license(&mut self, license_item: LicenseItem) {
        self.licenses.push(license_item);
    }
}