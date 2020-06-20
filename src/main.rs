extern crate virustotal;
extern crate colored;
extern crate clap;
extern crate core;

use virustotal::*;
use colored::{Colorize, ColoredString};

use clap::{Arg, App};
use std::fs::File;
use std::io::{Write, Read};

fn save_key(key: &str) -> std::io::Result<()> {
    let mut env = std::env::vars();
    let (_k, v) = env.find(
        |p: &(String, String)| {
            p.0 == "LOCALAPPDATA"
        }).unwrap();
    let mut file = File::create(
        format!("{}{}", v, "/virustotal-client-key.txt"))
        .unwrap();
    file.write_fmt(format_args!("{}", key))
}

fn read_key() -> String {
    let mut env = std::env::vars();
    let (_, v) = env.find(
        |p: &(String, String)| {
            p.0 == "LOCALAPPDATA"
        }).unwrap();
    let mut file = match File::open(format!("{}{}", v, "/virustotal-client-key.txt")) {
        Ok(f) => f,
        Err(e) => panic!("Key wasn't found in {}. {}", v, e)
    };
    let mut key = String::new();
    file.read_to_string(&mut key);
    key
}

fn main() -> std::io::Result<()> {
    let matches = App::new("VirusTotal client")
        .version("0.0.1")
        .author("Minium2, CbIpok")
        .about("Command line VirusTotal client")
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .takes_value(true)
            .about("file to check"))
        .arg(Arg::with_name("key")
            .short('k')
            .long("key")
            .takes_value(true)
            .about("Set VirusTotal api key. Obtain this key on official site"))
        .get_matches();

    let file = match matches.value_of("file") {
        Some(elem) => elem,
        None => {
            println!("{}", "Use --help command".red().to_string());
            return Ok(())
        }
    };
    println!("Start scan: {}", file.yellow());

    let api_key = match matches.value_of("key") {
        None => read_key(),
        Some(api) => {
            save_key(api);
            api.to_string()
        }
    };
    let vt =  VtClient::new(api_key.as_str());
    let res = vt.scan_file(file);
    println!("{}", vt.report_file(&res.scan_id.unwrap()));
    println!("{}", "Done".green());
    Ok(())
}