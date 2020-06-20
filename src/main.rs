extern crate virustotal;
extern crate colored;

use virustotal::*;
use colored::Colorize;

fn main() {
    let api = "639e80fe2d2d7a65257affc99d1640b04b53a88c7408788bc766e1bd3afe90be";
    let file = "D:\\Download\\[R.G. Mechanics] Dark Souls 3 - GOTY\\setup.exe ";
    let vt = VtClient::new(api);
    let res = vt.scan_file(file);
    println!("{}", vt.report_file(&res.scan_id.unwrap()));
    println!("{}", "Done...".green());
}