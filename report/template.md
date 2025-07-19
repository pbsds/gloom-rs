---
# These are meta-variables defined with YAML syntax, change them as you wish.
# see https://pandoc.org/MANUAL.html#variables
title: TDT4195 Assignment X
author:
- Gyrd Bannamule Gyrdsson
- Gjavleik Britonis Podebusk
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: "List of Contents"
toc-depth: 2
numbersections: true
colorlinks: true
links-as-notes: true
# The document is following the break written using Markdown syntax
---

<!--
This is a HTML-style comment, not visible in the final PDF.
-->

# Heading

## Subheading

### Subsubheading

This is a paragraph.
This is the same paragraph.

This is a new paragraph, with *italic*, **bold**, and `inline code` formatting.
It is possible to use special classes to format text: [this is a test]{.smallcaps}.

```rust
//this is a code block with rust syntax highlighting
println!("Hello, {}", 42);
```

[This](https://www.ntnu.no) is a link.
[This][] is also a link. <!-- defined below -->
This[^this_is_a_unique_footnote_label] is a footnote.

[This]: https://www.uio.no
[^this_is_a_unique_footnote_label]: In footnotes you can write anything tangentially related.

* This
* is
* a
* bullet
* list

1. This
1. is
1. a
1. numbered
1. list
    a. with
    a. sub
    a. list

       with multiple paragraphs

This is still on the first page

\clearpage

<!--
Above is a raw LaTeX statement.
Those are included when exporting to LaTeX or PDF, and ignored when exporting to HTML.
-->

This is on the second page

i) Yo
i) Yo
i) Yo

This
: is a definition

> this is a
block quote

This is a paragraph with inline \LaTeX\ math: $\frac{1}{2}$.
Below is a math block:

$$
    \int_{a}^{b} f(x)dx
$$

| This | is  | a   | table |
| ---- | --- | --- | ----- |
| 1    | 2   | 3   | 4     |
| 5    | 6   | 7   | 8     |

: This is a table caption

This is an inline image with set height:
![](images/logo.png){height=5em}

Below is a figure (i.e. an image with a caption).
It floats and may as a result move to a different page depending on the layout.
Use the `pandoc-crossref` filter to reference figures, tables and equations.

![
    Image with caption
](images/logo.png)
