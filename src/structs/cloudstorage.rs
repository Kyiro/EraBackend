use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct SystemEntry {
    pub unique_filename: String,
    pub filename: String,
    pub hash: String,
    pub hash256: String,
    pub length: usize,
    pub content_type: String,
    pub uploaded: String,
    pub storage_type: String,
    pub do_not_cache: bool,
}

impl SystemEntry {
    pub fn new(name: String, data: String) -> Self {
        let content = String::into_bytes(data);
            
        let mut sha1 = Sha1::new();
        let mut sha256 = Sha256::new();
        sha1.update(&content);
        sha256.update(&content);
        let sha1 = sha1.finalize();
        let sha256 = sha256.finalize();

        let content = String::from_utf8(content).unwrap();

        SystemEntry {
            unique_filename: name.to_string(),
            filename: name.to_string(),
            hash: format!("{:x}", sha1),
            hash256: format!("{:x}", sha256),
            length: content.len(),
            content_type: String::from("application/octet-stream"),
            uploaded: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            storage_type: String::from("S3"),
            do_not_cache: true,
        }
    }
}
