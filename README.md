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

### Meta tags

There are some meta tags which are converted on generation time

| meta tag | info |
| --- | --- |
| `<date>` | Converted to formatted date. Format is YYYYMMDD. (Example: 20201025) |

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
