#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate core;
extern crate serde;

use std::collections::HashMap;
use core::fmt;
use serde::export::Formatter;

/// A set of scanning an URL
pub mod url;
/// A set of reporting domain
pub mod domain;
/// A set of repoting ip
pub mod ip;
/// A set of scanning a file
pub mod file;
/// A set of putting a comment
pub mod comment;

#[derive(Debug,Deserialize)]
pub struct CommentPutResponse {
    pub response_code: i32,
    pub verbose_msg: String
}

#[derive(Debug,Deserialize)]
pub struct Comment {
    pub date: String,
    pub comment: String
}

#[derive(Debug,Deserialize)]
pub struct CommentGetResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: String,
    pub comments: Vec<Comment>
}

#[derive(Debug,Deserialize)]
pub struct UrlScanResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub scan_id: Option<String>,
    pub scan_date: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct UrlReportResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub scan_date: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>,
    pub filescan_id: Option<String>,
    pub positives: Option<u32>,
    pub total: Option<u32>,
    pub scans: Option<HashMap<String, Scan>>
}

#[derive(Debug,Deserialize)]
pub struct Scan {
    pub detected: Option<bool>,
    pub version: Option<String>,
    pub result: Option<String>,
    pub update: Option<String>,
    pub detail: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct FileScanResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub sha256: Option<String>,
    pub permalink: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct FileRescanResponse {
    pub response_code: i32,
    //pub verbose_msg: Option<String>,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub permalink: Option<String>,
    pub sha256: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct FileReportResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub scan_date: Option<String>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>,
    pub filescan_id: Option<String>,
    pub positives: Option<u32>,
    pub total: Option<u32>,
    pub scans: Option<HashMap<String, Scan>>
}

#[derive(Debug,Deserialize)]
pub struct ReportResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub scan_date: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>,
    pub filescan_id: Option<String>,
    pub positives: Option<u32>,
    pub total: Option<u32>,
    pub scans: Option<HashMap<String, Scan>>
}

#[derive(Debug,Deserialize)]
pub struct DomainResolutions {
    pub last_resolved: String,
    pub ip_address: String
}

#[derive(Debug,Deserialize)]
pub struct IpAddressResolutions {
    pub last_resolved: String,
    pub hostname: String
}

#[derive(Debug,Deserialize)]
pub struct DetectedUrls {
    pub url: String,
    pub positives: u32,
    pub total: u32,
    pub scan_date: String,
}

#[derive(Debug,Deserialize)]
pub struct ReferrerSample {
    pub date: Option<String>,
    pub positives: Option<i32>,
    pub total: Option<i32>,
    pub sha256: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct DomainReportResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resolutions: Vec<DomainResolutions>,
    pub detected_urls: Vec<DetectedUrls>,
    pub subdomains: Vec<String>,
    pub categories: Vec<String>,
    //pub domain_siblings: Vec<String>,
    pub undetected_referrer_samples: Option<Vec<ReferrerSample>>,
    pub undetected_downloaded_samples: Option<Vec<ReferrerSample>>,
    pub detected_referrer_samples: Option<Vec<ReferrerSample>>,
    pub detected_downloaded_samples: Option<Vec<ReferrerSample>>,
    pub whois_timestamp: i32,
    pub whois: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct IpAddressReportResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub country: Option<String>,
    pub asn: Option<u64>,
    pub detected_downloaded_samples: Option<Vec<ReferrerSample>>,
    pub undetected_downloaded_samples: Option<Vec<ReferrerSample>>,
    pub resolutions: Vec<IpAddressResolutions>,
    pub detected_urls: Vec<DetectedUrls>,
}

#[derive(Copy, Clone)]
pub struct VtClient<'a> {
    api_key: &'a str,
    endpoint: &'a str
}
impl <'a>VtClient<'a> {
    pub fn new(api_key: &'a str) -> Self {
        VtClient{api_key: api_key, endpoint: "https://www.virustotal.com/vtapi/v2"}
    }
}

//Todo unimplemented yet
impl fmt::Display for Scan {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let res = match &self.result {
            Some(elem) => elem,
            None => ""
        };
        let detail = match &self.detail {
            Some(elem) => elem,
            None => ""
        };
        write!(f, "Result: {} Detail: {}", res, detail)
    }
}

impl fmt::Display for FileReportResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let scan = match &self.scans {
            Some(el) => el,
            None => return Ok(())
        };
        for (i, j) in scan {
            write!(f, "{}, {}", i, j);
        }
        Ok(())
    }
}