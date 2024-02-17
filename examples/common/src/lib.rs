use vlks::licensing::LicenseRepository;
use vlks::licensing::LicenseItem;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MyLicenseItem {
    license: String,
    attributes: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct MyLicenseRepository {
    licenses: Vec<MyLicenseItem>,
}

impl MyLicenseItem {
    pub fn new(license: String, attributes: HashMap<String, String>) -> MyLicenseItem {
        MyLicenseItem {
            license,
            attributes
        }
    }
}

impl MyLicenseRepository {
    pub fn new(licenses: Vec<MyLicenseItem>) -> MyLicenseRepository {
        MyLicenseRepository {
            licenses: vec![]
        }
    }
}

impl LicenseRepository<MyLicenseItem> for MyLicenseRepository {
    fn add_license(&mut self, license_item: MyLicenseItem) {
        self.licenses.push(license_item);
    }
    fn get_license(&self, license: String) -> Option<MyLicenseItem> {
        for item in &self.licenses {
            if item.clone().get_license() == license {
                return Some(item.clone());
            }
        }
        None
    }
    fn get_licenses(self) -> Vec<MyLicenseItem> {
        self.licenses
    }
    fn delete_license(&mut self, license_item: MyLicenseItem) {
        self.licenses.push(license_item);
    }
}

impl LicenseItem for MyLicenseItem {
    fn get_license(self) -> String {
        self.license
    }
    fn get_attributes(self) -> HashMap<String, String> {
        self.attributes
    }
}