use crate::option::Option;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn parse_option() -> Result<Option> {
    let mut template_file = File::open(format!("./{}", Option::CONFIG_FILENAME))
        .with_context(|| format!("{} is not found", Option::CONFIG_FILENAME))?;
    let mut contents = String::new();

    template_file
        .read_to_string(&mut contents)
        .with_context(|| format!("Unexpected error while reading {}", Option::CONFIG_FILENAME))?;

    let option_map: HashMap<String, String>;

    if let Some(kind) = std::env::args().nth(1) {
        let template_map: HashMap<String, HashMap<String, String>> =
            serde_yaml::from_str(&contents).with_context(|| {
                format!("Unexpected error while parsing {}", Option::CONFIG_FILENAME)
            })?;

        option_map = template_map
            .get(&kind)
            .with_context(|| format!("\"{}\" is not defined in {}", kind, Option::CONFIG_FILENAME))?
            .clone();
    } else {
        option_map = serde_yaml::from_str(&contents).with_context(|| {
            format!("Unexpected error while parsing {}", Option::CONFIG_FILENAME)
        })?;
    };

    Ok(Option::new(
        read_option(&option_map, "filename")?,
        read_option(&option_map, "body")?,
        option_map.get("dir"),
    ))
}

fn read_option(option_map: &HashMap<String, String>, field: &str) -> Result<String> {
    Ok(option_map
        .get(field)
        .with_context(|| {
            format!(
                "\"{}\" is not defined in {}",
                field,
                Option::CONFIG_FILENAME
            )
        })?
        .to_owned())
}
