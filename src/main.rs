use std::fs::{File, OpenOptions};
use std::io::prelude::*;

struct DocumentOption {
    filename: String,
    body: String,
}

impl DocumentOption {
    fn convert(&mut self) {
        let conv = |s: &str| {
            s.replace(
                "<date>",
                &chrono::Local::today().format("%Y%m%d").to_string(),
            )
        };

        conv(&mut self.filename);
        conv(&mut self.body);
    }
}

fn main() {
    let mut template_file = File::open("./gendoc.yaml").expect("gendoc.yaml is not found");
    let mut contents = String::new();

    template_file.read_to_string(&mut contents).unwrap();

    let template_map: std::collections::BTreeMap<String, String> =
        serde_yaml::from_str(&contents).unwrap();

    let filename = template_map.get("filename").unwrap();
    let body = template_map.get("body").unwrap();

    let mut document_option = DocumentOption {
        filename: filename.to_owned(),
        body: body.to_owned(),
    };

    document_option.convert();

    generate(&document_option);

    println!("Document generated: {}", filename);
}

fn generate(document_option: &DocumentOption) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&document_option.filename)
        .expect(&format!(
            "target filename {} already exists",
            document_option.filename
        ));

    file.write_all(document_option.body.as_bytes()).unwrap();
}
