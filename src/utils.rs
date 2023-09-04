#![allow(unused)]
use failure::{bail, Fallible};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use url::{ParseError, Url};


pub fn parse_url(url: &str) -> Result<Url,ParseError> {
    match Url::parse(url){
        Ok(url) => Ok(url),
        Err(error) if error == ParseError::RelativeUrlWithoutBase => {
            let url_with_base = format!("{}{}", "http://", url);
            Url::parse(&url_with_base)
        }
        Err(error) => Err(error),
    }
}

pub fn gen_error(msg:String) -> Fallible<()>{
    bail!(msg)
}

pub fn print(string:String,quiet_mode:bool){
    if !quiet_mode{
        println!("{}",string);
    }
}