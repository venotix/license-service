use std::any::Any;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LicenseItem {
    pub license: String,
    pub attributes: HashMap<String, String>
}

pub trait LicenseRepository {
    fn add_license(&mut self, license_item: LicenseItem);
    fn delete_license(&mut self, license_item: LicenseItem);
    fn get_licenses(self) -> Vec<LicenseItem>;
    fn get_license(&self, license: String) -> Option<LicenseItem>;
}