use std::fs::File;
use std::io::Write;

use crate::scanner::engine::ScanResult;

pub fn save_recon(result: &ScanResult) {

    let json = serde_json::to_string_pretty(result)
        .unwrap();

    let mut file = File::create("defense-lab/recon.json")
        .unwrap();

    file.write_all(json.as_bytes())
        .unwrap();
}
