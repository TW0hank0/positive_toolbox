
#[derive(Debug)]
pub struct LicenseInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub authors: Vec<&'static str>,
}

pub fn get_licenses() -> Vec<LicenseInfo> { 
return vec![

LicenseInfo {
    name: "altgraph",
    version: "0.17.5",
    license: "MIT License",
    authors: vec![],
},

LicenseInfo {
    name: "packaging",
    version: "26.0",
    license: "Apache-2.0 OR BSD-2-Clause",
    authors: vec![],
},

LicenseInfo {
    name: "pefile",
    version: "2024.8.26",
    license: "MIT",
    authors: vec![],
},

LicenseInfo {
    name: "pyinstaller",
    version: "6.19.0",
    license: "GNU General Public License v2 (GPLv2)",
    authors: vec![],
},

LicenseInfo {
    name: "pyinstaller-hooks-contrib",
    version: "2026.0",
    license: "Apache Software License; GNU General Public License v2 (GPLv2)",
    authors: vec![],
},

LicenseInfo {
    name: "pywin32-ctypes",
    version: "0.2.3",
    license: "BSD-3-Clause",
    authors: vec![],
},];}