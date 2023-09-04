#![allow(unused)]
mod utils;

use clap::{App, Arg};
use reqwest::blocking::Client;
use failure::{format_err, Fallible};

fn main(){
    let matches = App::new("Rget")
    .version("0.1.0")
    .author("eileenlcb")
    .about("A simple wget clone")
    .arg(Arg::with_name("url")
        .help("The url to download")
        .required(true)
        .index(1))
        .help("Sets the output file to use")
    .get_matches();

    let url = matches.value_of("url").unwrap();
    println!("url: {}", url)
}


fn download(target:&str,quiet_mode:bool)->Result<(),Box<dyn::std::error::Error>>{
    let url = utils::parse_url(target)?;
    let client = Client::new();
    let mut resp = client.get(url.as_ref()).send()?;
    
    match url.scheme() {
        "ftp" => ftp_download(),
        "http" | "https" => http_download(),
        _ => utils::gen_error(format!("unsupported url scheme '{}'", url.scheme())),
    };

    Ok(())
}

fn ftp_download() -> Fallible<()> {
    Ok(())
}

fn http_download() -> Fallible<()>{
    Ok(())
}