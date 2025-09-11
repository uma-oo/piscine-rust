use std::fs::{ File };

pub fn open_file(s: &str) -> File {
    match File::open(s.to_string()) {
        Ok(file) => file,
        Err(err) => panic!("{}",err.to_string()),
    }
}
