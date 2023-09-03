use clap::{App, Arg};

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


