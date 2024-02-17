use std::collections::HashMap;
use std::sync::Arc;
use example_common::{MyLicenseItem, MyLicenseRepository};
use vlks::licensing::{LicenseRepository, LicenseItem};
use uuid::Uuid;

fn main() {
    let mut system: MyLicenseRepository = MyLicenseRepository::new(vec![]);

    &system.add_license(MyLicenseItem::new("c97283f4-06ee-46c1-8e39-d337483f4eb1".to_string(), HashMap::new()));
    &system.add_license(MyLicenseItem::new("c97283f4-06ee-46c1-8e39-d337483f4eb2".to_string(), HashMap::new()));
    &system.add_license(MyLicenseItem::new("c97283f4-06ee-46c1-8e39-d337483f4eb3".to_string(), HashMap::new()));

    println!("{:?}", &system.get_license("c97283f4-06ee-46c1-8e39-d337483f4eb1".to_string()).unwrap());

    for license in system.get_licenses() {
        println!("{}", license.get_license());
    }
}