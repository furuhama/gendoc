pub struct Option {
    pub filename: String,
    pub body: String,
    pub dir: String,
    pub path: String,
}

impl Option {
    pub const CONFIG_FILENAME: &'static str = "gendoc.yaml";

    pub fn new(filename: String, body: String, dir: std::option::Option<&String>) -> Self {
        let dir = match dir {
            Some(s) => format!("./{}/", s),
            None => "./".to_owned(),
        };
        let path = format!("{}{}", dir, filename);

        Self {
            filename,
            body,
            dir,
            path,
        }
    }

    pub fn convert(&mut self) {
        let filename = Self::convert_field(&self.filename);
        let body = Self::convert_field(&self.body);
        let dir = Self::convert_field(&self.dir);
        let path = format!("{}{}", dir, filename);

        self.filename = filename;
        self.body = body;
        self.dir = dir;
        self.path = path;
    }

    fn convert_field(s: &str) -> String {
        let result = Self::convert_meta_tag(
            "date",
            |s: &str| chrono::Local::today().format(s).to_string(),
            "%Y%m%d",
            s,
        );

        let result = Self::convert_meta_tag(
            "datetime",
            |s: &str| chrono::Local::now().format(s).to_string(),
            "%Y%m%d%H%M%S",
            &result,
        );

        result
    }

    fn convert_meta_tag<F>(tag_name: &str, formatter: F, default_arg: &str, s: &str) -> String
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_new() {
        let option = Option::new("filename".to_owned(), "body".to_owned(), None);
        assert_eq!(option.dir, "./");
        assert_eq!(option.path, "./filename");

        let option = Option::new(
            "filename".to_owned(),
            "body".to_owned(),
            Some(&"dir".to_owned()),
        );
        assert_eq!(option.dir, "./dir/");
        assert_eq!(option.path, "./dir/filename");

        let option = Option::new(
            "filename".to_owned(),
            "body".to_owned(),
            Some(&"dir/dir2".to_owned()),
        );
        assert_eq!(option.dir, "./dir/dir2/");
        assert_eq!(option.path, "./dir/dir2/filename");
    }

    #[test]
    fn test_option_convert_field() {
        let pair = [
            ("without meta tag", "without meta tag"),
            (
                "<date>",
                &chrono::Local::today().format("%Y%m%d").to_string(),
            ),
            (
                "<date:%Y-%m-%d>",
                &chrono::Local::today().format("%Y-%m-%d").to_string(),
            ),
            (
                "<datetime>",
                &chrono::Local::now().format("%Y%m%d%H%M%S").to_string(),
            ),
            (
                "<datetime:%Y-%m-%d-%H%M%S>",
                &chrono::Local::now().format("%Y-%m-%d-%H%M%S").to_string(),
            ),
            (
                "<date:%Y-%m-%d><datetime:%Y-%m-%d-%H%M%S>",
                &format!(
                    "{}{}",
                    chrono::Local::today().format("%Y-%m-%d").to_string(),
                    chrono::Local::now().format("%Y-%m-%d-%H%M%S").to_string()
                ),
            ),
        ];

        for (actual, expect) in pair.iter() {
            assert_eq!(Option::convert_field(*actual), *expect);
        }
    }
}
