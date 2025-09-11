use std::path::Path;
use std::fs::{ OpenOptions };
use std::io::Write;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new().read(true).write(true).append(true).create(true).open(path).unwrap();
    match file.write_all(content.as_bytes()) {
        Ok(())=> {}, 
        Err(err)=> panic!("{}", err.to_string()),
    }

}
