use std::error::Error;

use filehash_rs::filehash;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Path to the file:");

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string)?;

    let result = filehash(&mut input_string);

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
}
