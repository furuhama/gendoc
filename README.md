# gendoc

[![gendoc on crates.io][cratesio-image]][cratesio]
![Rust](https://github.com/furuhama/competitive/workflows/Rust/badge.svg)

[cratesio-image]: https://img.shields.io/crates/v/gendoc.svg
[cratesio]: https://crates.io/crates/gendoc

Simple document generator with template yaml file

## Usage

Run `gendoc` with `gendoc.yaml` file

```
$ cat gendoc.yaml
filename: test_<datetime>.rb
body: |
  # frozen_string_literal: true

  require 'json'

  puts JSON.parse("{}")
$ gendoc
Document generated: ./test_20200927023752.rb
$ cat test_20200927023752.rb
# frozen_string_literal: true

require 'json'

puts JSON.parse("{}")
$ ruby test_20200927023752.rb
{}
```

### Settings

There are some setting parameters.

| parameter | info | required |
| :---: | --- | :---: |
| `filename` | A filename of a generated document. | ○ |
| `body` | A body of a generated document. | ○ |
| `dir` | A directory of a generated document. It will be parsed as a relative path (ex: `tmp` -> `./tmp`). `gendoc` does NOT create a new directory on generation time. | |

### Meta tags

There are some meta tags which are converted on generation time.

| meta tag | info |
| :---: | --- |
| `<date>` | Converted to formatted date. Format is `YYYYmmdd`. (ex: `<date>` -> `19720719`) |
| `<date:_format_string_>` | Same as `<date>`, and you can pass an format string. (ex: `<date:%Y-%m-%d>` -> `1972-07-19`) |
| `<datetime>` | Converted to formatted datetime. Format is `YYYYmmddHHMMSS`. (ex: `<datetime>` -> `19720719000545`) |
| `<datetime:_format_string_>` | Same as `<datetime>`, and you can pass an format string. (ex: `<datetime:%Y-%m-%d-%H%M%S>` -> `1972-07-19-000545`) |
| `<input>` | Converted to input text. You should pass text from STDIN on generation time. |

To get more info abount format string, see [here](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

### Multiple settings

Here's a sample `gendoc.yaml` to set multiple settings.

```yaml
sample1:
  filename: sample1.txt
  body: |
    this is a 1st setting
saple2:
  filename: sample2.txt
  body: |
    this is a 2nd setting
```

Run `gendoc` with a setting name.

```
$ gendoc sample2
Document generated: sample2.txt
$ cat sample2.txt
this is a 2nd setting
```

### TODO

- Be able to set argument to `<input>` tag. (`<input>` with the same argument will be filled with the same value.)
