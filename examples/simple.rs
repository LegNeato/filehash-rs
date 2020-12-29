use filehash_rs::filehash;

fn main() {
    println!("Path to the file:");

    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string).unwrap();

    let result = filehash(&mut input_string);

    match result {
        Ok(value) => {
            println!("Result: {}", value);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
