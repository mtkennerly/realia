// Inspired by and updated from: https://stackoverflow.com/a/41638455

use std::io::Write;

const CRATE_INFO_TEMPLATE: &str = r#"
pub struct CrateInfo<'a> {
    name: &'a str,
    version: &'a str,
    source: Option<&'a str>,
}
"#;

fn main() {
    let raw_lock = std::fs::read_to_string("Cargo.lock").unwrap();
    let parsed_lock = raw_lock.parse::<toml::Value>().unwrap();

    let mut packages = Vec::new();
    for package in parsed_lock["package"].as_array().unwrap() {
        let package = package.as_table().unwrap();
        packages.push((
            package["name"].as_str().unwrap(),
            package["version"].as_str().unwrap(),
            match package.get("source") {
                Some(source) => Some(source.as_str().unwrap()),
                None => None,
            },
        ));
    }
    packages.sort();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut crates_file = std::fs::File::create(&std::path::Path::new(&out_dir).join("crate_info.rs")).unwrap();
    crates_file.write_all(CRATE_INFO_TEMPLATE.as_bytes()).unwrap();
    crates_file
        .write_all(format!("pub const CRATES: [CrateInfo; {}] = [", packages.len()).as_ref())
        .unwrap();
    for package in packages {
        crates_file
            .write_all(
                format!(
                    "CrateInfo {{ name: {:?}, version: {:?}, source: {:?} }},\n",
                    package.0, package.1, package.2
                )
                .as_ref(),
            )
            .unwrap();
    }
    crates_file.write_all("];".as_ref()).unwrap();
}
