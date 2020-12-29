use filehash_rs::filehash;
extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("filehash-rs")
        .version("1.0.0")
        .author("Daniel Sonne Lehnberg <doddy@doddy.se")
        .about("A small (3kb), fast library for hashing files written in Rust.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Assigns a path to the file that should be hashed.")
                .takes_value(true),
        )
        .get_matches();
    let file = matches.value_of("file");
    if let Some(file_value) = file {
        let mut file_string = file_value.to_string();
        let result = filehash(&mut file_string);

        match result {
            Ok(value) => {
                println!("{}", value);
                return Ok(());
            }
            Err(err) => {
                println!("Error: {}", err);
                return Err(Box::new(err));
            }
        }
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No valid file path was provided.",
        )));
    }
}
