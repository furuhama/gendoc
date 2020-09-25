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
