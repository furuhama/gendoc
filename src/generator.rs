use crate::document_option::DocumentOption;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn generate(document_option: &DocumentOption) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&document_option.filename)
        .expect(&format!(
            "target filename {} already exists",
            document_option.filename
        ));

    file.write_all(document_option.body.as_bytes()).unwrap();

    println!("Document generated: {}", document_option.filename);
}
