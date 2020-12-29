use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{Error, Read},
    path::Path,
};

/// ```rust
/// // A simple example of a program that queries the user
/// // for a file to hash and prints the result.
/// # use std::error::Error;
/// # use filehash_rs::filehash;
/// fn main() -> Result<(), Box<dyn Error>> {
///    let mut input_string = String::new();
///    println!("Path to the file:");
///
///    std::io::stdin().read_line(&mut input_string)?;
/// #  input_string = String::from("./mock/mock.txt");
///
///    let result = filehash(&mut input_string);
///
///    match result {
///        Ok(value) => {
///            println!("{}", value);
///            return Ok(());
///        }
///        Err(err) => {
///            println!("Error: {}", err);
///            return Err(Box::new(err));
///        }
///    }
/// }
/// ```

pub fn filehash(file_path: &mut String) -> Result<u64, Error> {
    let tmp = file_path.trim();
    let path_input = Path::new(&tmp);

    let path_canonicalized = path_input.canonicalize()?;
    let path_os_string = path_canonicalized.as_os_str();
    let mut buffer: Vec<u8> = Vec::new();

    let mut f = File::open(path_os_string)?;
    f.read_to_end(&mut buffer)?;

    let mut hasher = DefaultHasher::new();
    buffer.hash(&mut hasher);
    let finished_hash = hasher.finish();
    return Ok(finished_hash);
}

#[cfg(test)]
mod tests {
    use super::filehash;

    #[test]
    fn it_generates_consistent_results_png() {
        let mut mock_file1 = String::from("./mock/mock.png");
        let mut mock_file2 = String::from("./mock/mock.png");
        let result1 = filehash(&mut mock_file1);
        let result2 = filehash(&mut mock_file2);
        assert_eq!(result1.unwrap(), result2.unwrap());
    }
    #[test]
    fn it_generates_consistent_results_txt() {
        let mut mock_file1 = String::from("./mock/mock.txt");
        let mut mock_file2 = String::from("./mock/mock.txt");
        let result1 = filehash(&mut mock_file1);
        let result2 = filehash(&mut mock_file2);
        assert_eq!(result1.unwrap(), result2.unwrap());
    }
    #[test]
    fn it_generates_consistent_results_json() {
        let mut mock_file1 = String::from("./mock/mock.json");
        let mut mock_file2 = String::from("./mock/mock.json");
        let result1 = filehash(&mut mock_file1);
        let result2 = filehash(&mut mock_file2);
        assert_eq!(result1.unwrap(), result2.unwrap());
    }
}
