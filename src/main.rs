use std::fs::{File, OpenOptions};
use std::io::prelude::*;

struct DocumentOption {
    filename: String,
    body: String,
}

impl DocumentOption {
    fn convert(&mut self) {
        self.filename = Self::_convert_field(&self.filename);
        self.body = Self::_convert_field(&self.body);
    }

    // private
    fn _convert_field(s: &str) -> String {
        let result = DocumentOption::_convert_meta_tag(
            "date",
            |s: &str| chrono::Local::today().format(s).to_string(),
            "%Y%m%d",
            s,
        );

        let result = DocumentOption::_convert_meta_tag(
            "datetime",
            |s: &str| chrono::Local::now().format(s).to_string(),
            "%Y%m%d%H%M%S",
            &result,
        );

        result
    }

    // private
    fn _convert_meta_tag<F>(tag_name: &str, formatter: F, default_arg: &str, s: &str) -> String
    where
        F: Fn(&str) -> String,
    {
        let mut result = s.replace(&format!("<{}>", tag_name), &formatter(default_arg));

        // `.*?` is a shortest match
        let re = regex::Regex::new(&format!("<{}:.*?>", tag_name)).unwrap();

        if let Some(caps) = re.captures(&result.clone()) {
            for cap in caps.iter() {
                if let Some(c) = cap {
                    let matched_str = c.as_str();
                    let format_pattern = &matched_str[tag_name.len() + 2..matched_str.len() - 1];

                    result.replace_range(c.range(), &formatter(format_pattern));
                }
            }
        };

        result
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
    fn test_document_option_convert_field() {
        assert_eq!(
            DocumentOption::_convert_field("without meta tag"),
            "without meta tag"
        );
        assert_eq!(
            DocumentOption::_convert_field("<date>"),
            chrono::Local::today().format("%Y%m%d").to_string()
        );
        assert_eq!(
            DocumentOption::_convert_field("<date:%Y-%m-%d>"),
            chrono::Local::today().format("%Y-%m-%d").to_string()
        );
        assert_eq!(
            DocumentOption::_convert_field("<datetime>"),
            chrono::Local::now().format("%Y%m%d%H%M%S").to_string()
        );
        assert_eq!(
            DocumentOption::_convert_field("<datetime:%Y-%m-%d-%H%M%S>"),
            chrono::Local::now().format("%Y-%m-%d-%H%M%S").to_string()
        );
        assert_eq!(
            DocumentOption::_convert_field("<date:%Y-%m-%d><datetime:%Y-%m-%d-%H%M%S>"),
            format!(
                "{}{}",
                chrono::Local::today().format("%Y-%m-%d").to_string(),
                chrono::Local::now().format("%Y-%m-%d-%H%M%S").to_string()
            )
        );
    }
}
