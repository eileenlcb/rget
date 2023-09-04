#![allow(unused)]
mod utils;

use std::fs::File;
use std::io::copy;

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
    println!("url: {}", url);
    match download(url, false) {
        Ok(_) => println!("Downloaded {} successfully!", url),
        Err(e) => println!("Error downloading {}: {}", url, e),
    };
}


fn download(target:&str,quiet_mode:bool)->Result<(),Box<dyn::std::error::Error>>{
    let url = utils::parse_url(target)?;
    let client = Client::new();
    let mut resp = client.get(url.as_ref()).send()?;
    
    match url.scheme() {
        "ftp" => ftp_download(),
        "http" | "https" => http_download(target),
        _ => utils::gen_error(format!("unsupported url scheme '{}'", url.scheme())),
    };

    Ok(())
}

fn ftp_download() -> Fallible<()> {
    Ok(())
}

fn http_download(url:&str) -> Fallible<()>{
    let resp = Client::new().get(url).send()?;
    let headers = resp.headers();
    let server_supports_bytes = match headers.get("Accept-Ranges") {
        Some(val) => val == "bytes",
        None => false,
    };

    Ok(())
}

fn save_to_file(contents: &mut Vec<u8>, fname: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(fname).unwrap();
    copy(&mut contents.as_slice(), &mut file).unwrap();
    Ok(())
}