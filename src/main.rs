use std::fs::{File, OpenOptions};
use std::io::prelude::*;

struct DocumentOption {
    filename: String,
    body: String,
}

impl DocumentOption {
    fn convert(&mut self) {
        let conv = |s: &str| -> String {
            s.replace(
                "<date>",
                &chrono::Local::today().format("%Y%m%d").to_string(),
            )
        };

        self.filename = conv(&self.filename);
        self.body = conv(&self.body);
    }
}

fn main() {
    let mut document_option = parse_option();

    document_option.convert();

    generate(&document_option);
}

fn parse_option() -> DocumentOption {
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

    println!("Document generated: {}", document_option.filename);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_option_convert() {
        let mut document_option = DocumentOption {
            filename: "<date>".to_owned(),
            body: "body".to_owned(),
        };

        document_option.convert();

        assert_eq!(
            document_option.filename,
            chrono::Local::today().format("%Y%m%d").to_string()
        );
        assert_eq!(document_option.body, "body");
    }
}
