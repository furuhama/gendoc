use crate::option::Option;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn parse_option() -> Result<Option> {
    let mut template_file =
        File::open("./gendoc.yaml").with_context(|| "gendoc.yaml is not found")?;
    let mut contents = String::new();

    template_file
        .read_to_string(&mut contents)
        .with_context(|| "Unexpected error while reading gendoc.yaml")?;

    let option_map: HashMap<String, String>;

    if let Some(kind) = std::env::args().nth(1) {
        let template_map: HashMap<String, HashMap<String, String>> =
            serde_yaml::from_str(&contents)
                .with_context(|| "Unexpected error while parsing gendoc.yaml")?;

        option_map = template_map
            .get(&kind)
            .with_context(|| format!("{} is not defined in gendoc.yaml", kind))?
            .clone();
    } else {
        option_map = serde_yaml::from_str(&contents)
            .with_context(|| "Unexpected error while parsing gendoc.yaml")?;
    };

    Ok(Option {
        filename: read_option(&option_map, "filename")?,
        body: read_option(&option_map, "body")?,
        dir: match option_map.get("dir") {
            Some(s) => format!("./{}/", s),
            None => "./".to_owned(),
        },
    })
}

fn read_option(option_map: &HashMap<String, String>, field: &str) -> Result<String> {
    Ok(option_map
        .get(field)
        .with_context(|| format!("\"{}\" is not defined in gendoc.yaml", field))?
        .to_owned())
}
