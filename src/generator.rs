use crate::option::Option;
use anyhow::{format_err, Context, Result};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn generate(option: &Option) -> Result<()> {
    if !Path::new(&option.dir).exists() {
        return Err(format_err!(
            "target directory {} does not exist",
            option.dir
        ));
    }

    if Path::new(&option.path).exists() {
        return Err(format_err!("target file {} already exists", option.path));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&option.path)
        .with_context(|| "Unexpected error while creating a document file")?;

    file.write_all(option.body.as_bytes())
        .with_context(|| "Unexpected error while writing a document file")?;

    println!("Document generated: {}", option.path);

    Ok(())
}
