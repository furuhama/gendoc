use crate::option::Option;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn generate(option: &Option) {
    if !Path::new(&option.dir).exists() {
        panic!("target directory {} does not exist", option.dir);
    }
    if Path::new(&option.path()).exists() {
        panic!("target file {} already exists", option.path());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&option.path())
        .unwrap();

    file.write_all(option.body.as_bytes()).unwrap();

    println!("Document generated: {}", option.path());
}
