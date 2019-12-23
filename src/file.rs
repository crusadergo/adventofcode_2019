use std::fs::File;
use std::io::BufReader;

pub fn reader(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).unwrap();
    BufReader::new(file)
}
