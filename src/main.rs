#![allow(unused)]
mod utils;

use std::{fs::File, io::Read};
use std::io::copy;

use clap::{App, Arg};
use reqwest::blocking::{Client, Request,Response};
use failure::{format_err, Fallible};
use console::style;
use indicatif::{ProgressBar, ProgressStyle, HumanBytes};
// use reqwest::header::{Range, ByteRangeSpec, ContentLength, ContentType, AcceptRanges, RangeUnit};

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

    if resp.status().is_success(){
        let headers = resp.headers().clone();
        let ct_len = headers.get("Content-Length").unwrap().to_str().unwrap();
        let ct_type = headers.get("Content-Type").unwrap().to_str().unwrap();

        let len = ct_len.parse::<u64>().unwrap();
        let len_option = if len > 0 { Some(len) } else { None };

        utils::print(format!("Length: {} ({})", style(len).green(), style(format!("{}", HumanBytes(len))).red()), quiet_mode);
        utils::print(format!("Type: {}", style(ct_type).green()), quiet_mode);
        utils::print(format!("Saving to: {}", style(fname).green()), quiet_mode);

        let chunk_size = match len_option{
            Some(x) => x as usize/99,
            None => 1024usize,
        };


        let bar =  create_progress_bar(quiet_mode,fname,len_option);
        let mut buf = Vec::new();
        
        let mut count = 0;
        loop {
            count += 1;
            let mut buffer = vec![0; chunk_size];
            let bcount = resp.read(&mut buffer[..]).unwrap();
            buffer.truncate(bcount);
            if !buffer.is_empty() {
                buf.extend(buffer.iter()
                               .cloned());
                bar.inc(bcount as u64);
            } else {
                break;
            }
            
        }

        bar.finish();

        save_to_file(&mut buf, fname)?;

        
    }
    
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

fn create_progress_bar(quiet_mode:bool,msg:&str,length:Option<u64>) -> ProgressBar{
    let bar = match quiet_mode{
        true => ProgressBar::hidden(),
        false => match length{
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        }
    };
    bar.set_message(msg);

    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} {percent}% [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=>  ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}