extern crate virustotal;

use virustotal::*;

fn main() {
    let api = "639e80fe2d2d7a65257affc99d1640b04b53a88c7408788bc766e1bd3afe90be";
    let file = "C:\\Fraps\\fraps.exe ";
    let vt = VtClient::new(api);
    let res = vt.scan_file(file);
    println!("{}", vt.report_file(&res.scan_id.unwrap()));
}