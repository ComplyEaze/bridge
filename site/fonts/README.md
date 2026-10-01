# Fonts, vendored

Subsets of three variable fonts, each under the SIL Open Font License 1.1 (the licence texts, with
their copyright notices, are the `OFL-*.txt` files here). None declares a Reserved Font Name.

| Files | Family | Source |
| --- | --- | --- |
| `bricolage-wght-latin.woff2`, `bricolage-wght-rupee.woff2` | Bricolage Grotesque | npm `@fontsource-variable/bricolage-grotesque` 5.3.0, `files/bricolage-grotesque-latin-wght-normal.woff2` and `...-latin-ext-wght-normal.woff2` |
| `geist-latin.woff2`, `geist-rupee.woff2` | Geist | npm `@fontsource-variable/geist` 5.3.0, `files/geist-latin-wght-normal.woff2` and `...-latin-ext-wght-normal.woff2` |
| `geist-mono-latin.woff2`, `geist-mono-rupee.woff2` | Geist Mono | npm `@fontsource-variable/geist-mono` 5.3.0, `files/geist-mono-latin-wght-normal.woff2` and `...-latin-ext-wght-normal.woff2` |

Each `-latin` file is its source cut to the characters the pages use, and each `-rupee` file is the
latin-ext source cut to the rupee sign alone, with fonttools:

```sh
pyftsubset SOURCE.woff2 --flavor=woff2 --layout-features='*' --no-hinting --output-file=OUT.woff2 \
  --unicodes="U+0020-007E,U+00A0-00FF,U+0131,U+0152-0153,U+02C6,U+02DA,U+02DC,U+2010-2027,U+2030-203A,U+20AC,U+2122,U+2212"
pyftsubset SOURCE-latin-ext.woff2 --flavor=woff2 --layout-features='*' --no-hinting --output-file=OUT-rupee.woff2 --unicodes="U+20B9"
```
