#![allow(unused)]
mod utils;

use std::fs::File;
use std::io::copy;

use clap::{App, Arg};
use reqwest::blocking::Client;
use failure::{format_err, Fallible};
use console::style;

fn main(){
    let matches = App::new("Rget")
    .version("0.1.0")
    .author("eileenlcb")
    .about("A simple wget clone")
    .arg(Arg::with_name("FILE")
        .short("O")
        .long("output-document")
        .help("write documents to FILE")
        .required(false)
        .takes_value(true))
    .arg(Arg::with_name("URL")
        .help("The url to download")
        .required(true)
        .index(1))
        .help("Sets the output file to use")
    .get_matches();

    let url = matches.value_of("URL").unwrap();
    let file_name = matches.value_of("FILE");

    let resume_download = false;
    println!("url: {}", url);
    match download(url, false,file_name,resume_download) {
        Ok(_) => println!("Downloaded {} successfully!", url),
        Err(e) => println!("Error downloading {}: {}", url, e),
    };

}


fn download(target:&str,quiet_mode:bool,filename: Option<&str>, resume_download: bool)->Result<(),Box<dyn::std::error::Error>>{
    
    let fname = match filename{
        Some(name) => name,
        None => target.split("/").last().unwrap(),
    };
    
    let url = utils::parse_url(target)?;
    let client = Client::new();
    let mut resp = client.get(url.as_ref()).send()?;
    

    utils::print(format!("HTTP request sent... {} and additional text", style(format!("{}", resp.status())).green()),
quiet_mode);
    
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