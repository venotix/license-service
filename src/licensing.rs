use std::collections::HashMap;

pub trait LicenseItem {
    fn get_license(self) -> String;
    fn get_attributes(self) -> HashMap<String, String>;
}

pub trait LicenseRepository<T: LicenseItem> {
    fn add_license(&mut self, license_item: T);
    fn delete_license(&mut self, license_item: T);
    fn get_licenses(self) -> Vec<T>;
    fn get_license(&self, license: String) -> Option<T>;
}