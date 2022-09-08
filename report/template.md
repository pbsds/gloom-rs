---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Exercise X
author:
  - Gyrd Bannamule Gyrdsson
  - Gjavleik Britonis Podebusk
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: "Table of Contents"
toc-depth: 2
numbersections: true
header-includes:
# The `atkinson` font, requires 'texlive-fontsextra' on arch or the 'atkinson' CTAN package
# Uncomment this line to enable:
#- '`\usepackage[sfdefault]{atkinson}`{=latex}'
colorlinks: true
links-as-notes: true
# The document is following this break is written using "Markdown" syntax
---

<!--
This is a HTML-style comment, not visible in the final PDF.
-->

# Heading

## Subheading

### Subsubheading

This is a paragraph.
This is the same paragraph.

This is a new paragraph, with _italic_, **bold**, and `inline code` formatting.
It is possible to use special classes to format text: [this is a test]{.smallcaps}.

```rust
//this is a code block with rust syntax highlighting
println!("Hello, {}", 42);
```

[This](https://www.ntnu.no) is a link.
[This][] is also a link. <!-- defined below -->
This[^this_is_a_unique_footnote_label] is a footnote. <!-- defined below -->
This^[Footnotes can also be written inline] is also a footnote.

[this]: https://www.uio.no

[^this_is_a_unique_footnote_label]: In footnotes you can write anything tangentially related.

- This
- is
- a
- unordered
- list

1. This
1. is
1. a
1. ordered
1. list
   a. with
   a. sub
   a. list

   with multiple paragraphs

This is still on the first page

`\clearpage`{=latex}

<!--
Above is a raw LaTeX statement.
Those are included when exporting to LaTeX or PDF, and ignored when exporting to HTML.
-->

This is on the second page

i) Roman ordered list
i) Roman ordered list
i) Roman ordered list

This
: is a definition

> this is a
> block quote

This is a paragraph with _inline_ \LaTeX\ style math: $\frac{1}{2}$.
Below is a math _block_:

$$
    \int_{a}^{b} f(x)dx
$$

| This | is  | a   | table |
| ---- | --- | --- | ----- |
| 1    | 2   | 3   | 4     |
| 5    | 6   | 7   | 8     |

: This is a table caption

This is an inline image with a fixed height:
![](images/logo.png){height=5em}

Below is a _figure_ (i.e. an image with a caption).
It floats and may as a result move to a different page depending on the layout.

![
    Image with caption
](images/logo.png)

Enable and use the `pandoc-crossref` filter to reference figures, tables and equations.
