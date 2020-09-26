## gendoc

Simple document generator with template yaml file

### Usage

Run `gendoc` with `gendoc.yaml` file

```
$ cat gendoc.yaml
filename: test_<date>.txt
body: |
  hoge
  fuga
$ gendoc
Document generated: test_20200926.txt
$ cat test_20200926.txt
hoge
fuga
```

### Settings

There are some setting parameters.

| parameter | info | required |
| :---: | --- | :---: |
| `filename` | A filename of a generated document. | ○ |
| `body` | A body of a generated document. | ○ |
| `dir` | A directory of a generated document. It will be parsed as a relative path. (ex: `tmp` -> `./tmp`) `gendoc` does NOT create a new directory on generation time. | |

### Meta tags

There are some meta tags which are converted on generation time.

| meta tag | info |
| :---: | --- |
| `<date>` | Converted to formatted date. Format is `YYYYmmdd`. (ex: `<date>` -> `19720719`) |
| `<date:_format_string_>` | Same as `<date>`, and you can pass an format string. (ex: `<date:%Y-%m-%d>` -> `1972-07-19`) |
| `<datetime>` | Converted to formatted datetime. Format is `YYYYmmddHHMMSS`. (ex: `<datetime>` -> `19720719000545`) |
| `<datetime:_format_string_>` | Same as `<datetime>`, and you can pass an format string. (ex: `<datetime:%Y-%m-%d-%H%M%S>` -> `1972-07-19-000545`) |

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
