use crate::option::Option;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn generate(option: &Option) {
    if !Path::new(&option.dir).exists() {
        panic!("target directory {} does not exist", option.dir);
    }

    let path = option.path();
    if Path::new(&path).exists() {
        panic!("target file {} already exists", path);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();

    file.write_all(option.body.as_bytes()).unwrap();

    println!("Document generated: {}", path);
}
