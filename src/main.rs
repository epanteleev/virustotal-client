extern crate virustotal;
extern crate clap;

use virustotal::*;

fn main_old() {
    let api = "639e80fe2d2d7a65257affc99d1640b04b53a88c7408788bc766e1bd3afe90be";
    let file = "C:\\Fraps\\fraps.exe ";
    let vt = VtClient::new(api);
    let res = vt.scan_file(file);
    println!("{}", vt.report_file(&res.scan_id.unwrap()));
}

use clap::{Arg, App};

fn main() {
    let matches = App::new("virustotal-client")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .takes_value(true)
            .help("file to check"))
        .arg(Arg::with_name("key")
            .short('k')
            .long("key")
            .takes_value(true)
            .help("api-key"))
        .get_matches();

    let myfile:&str = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("key");
    match num_str {
        None => println!("no key"),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Ok"),
                Err(_) => println!(" Err"),
            }
        }
    }
}