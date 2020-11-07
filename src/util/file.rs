use std::io::Read;

pub fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}
