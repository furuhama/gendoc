use std::fs::{File, OpenOptions};
use std::io::prelude::*;

fn main() {
    let mut template_file = File::open("./gendoc.yaml").expect("gendoc.yaml is not found");
    let mut contents = String::new();

    template_file.read_to_string(&mut contents).unwrap();

    let template_map: std::collections::BTreeMap<String, String> =
        serde_yaml::from_str(&contents).unwrap();

    let filename = template_map.get("filename").unwrap();
    let body = template_map.get("body").unwrap();

    let filename = convert(filename);
    let body = convert(body);

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filename)
        .expect(&format!("target filename {} already exists", filename));

    file.write_all(body.as_bytes()).unwrap();

    println!("Document generated: {}", filename);
}

fn convert(s: &str) -> String {
    s.replace(
        "<date>",
        &chrono::Local::today().format("%Y%m%d").to_string(),
    )
}
