use crate::option::Option;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn generate(option: &Option) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&option.filename)
        .expect(&format!(
            "target filename {} already exists",
            option.filename
        ));

    file.write_all(option.body.as_bytes()).unwrap();

    println!("Document generated: {}", option.filename);
}
