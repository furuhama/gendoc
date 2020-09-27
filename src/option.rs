pub struct Option {
    pub filename: String,
    pub body: String,
    pub dir: String,
}

impl Option {
    pub fn path(&self) -> String {
        format!("{}{}", self.dir, self.filename)
    }

    pub fn convert(&mut self) {
        self.filename = Self::convert_field(&self.filename);
        self.body = Self::convert_field(&self.body);
        self.dir = Self::convert_field(&self.dir);
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
