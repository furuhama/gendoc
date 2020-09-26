use crate::document_option::DocumentOption;
use std::fs::File;
use std::io::prelude::*;

pub fn parse_option() -> DocumentOption {
    let mut template_file = File::open("./gendoc.yaml").expect("gendoc.yaml is not found");
    let mut contents = String::new();

    template_file.read_to_string(&mut contents).unwrap();

    let option_map: std::collections::BTreeMap<String, String>;

    if let Some(kind) = std::env::args().nth(1) {
        let template_map: std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, String>,
        > = serde_yaml::from_str(&contents).unwrap();

        option_map = template_map.get(&kind).unwrap().clone();
    } else {
        option_map = serde_yaml::from_str(&contents).unwrap();
    };

    DocumentOption {
        filename: option_map.get("filename").unwrap().to_owned(),
        body: option_map.get("body").unwrap().to_owned(),
    }
}
