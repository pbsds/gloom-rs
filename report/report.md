---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Exercise 1
author:
- Jonas Joshua Costa
- Noé Hirschauer
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
- '`\usepackage[sfdefault]{atkinson}`{=latex}'
colorlinks: true
links-as-notes: true
# The document is following this break is written using "Markdown" syntax
---

<!--
This is a HTML-style comment, not visible in the final PDF.
-->

# Task 1c

After implementing the VAO function and correctly wiring up the shaders, we were able to produce the images below with triangles.

![](img/face.png)

![](img/sierpinski.png)

# Task 2a

![](img/clipped_tri.png)

In the image above we can see that the triangle was **clipped** by OpenGL. In this case, the model is clipped to fit within -1 and 1 in all directions. Without clipping, OpenGL would try to render the whole scene even if it would not be visible in the final product. As a result, clipping improves performance in many cases.

Adjusting the offending vertices fixes the triangle as demonstrated below.

![](img/unclipped-tri.png)

# Task 2b

In most cases, the triangle simply does not show up in the final image. This the result of face culling which selectively renders one side of the triangle, usually the front. The front is determined by the order of vertices. A counter-clockwise arrangement indicates that we're looking at the front of the face. Examples:

0 --> 1 --> 2  
1 --> 2 --> 0  
2 --> 1 --> 0

Earlier in the given code, we can see the following line of code:

```rust
gl::Enable(gl::CULL_FACE);
```

If we remove this line any arrangement of indices renders just fine.

# Task 2c

## Question 1

*Why does the depth buffer need to be reset each frame?*

The buffer must cleared in scenes where something can move as the buffer would otherwise contain invalid data from the previous frame. As a result the buffer is always filled with the value that signifies the farthest distance.

## Question 2

*In which situation can the Fragment Shader be executed multiple times for the same pixel? (Assume we do not use multisampling.)*

When two objects overlap, the fragment shader is fully applied to all pixels of the first object, then to all pixels of the second one. This may or may not discard the previously calculated pixel.

## Question 3

*What are the two most commonly used types of Shaders? What are the responsibilities of each of them?*

The most common types of shaders are:

- **Vertex shaders**  
Modify existing geometry to achieve effects such as perspective.
- **Fragment shaders**  
Define the color of each pixel.
- **Geometry shader**  
Unlike the vertex shader, the geometry shader may emit any number of vertices.
- **Compute shader**  
General purpose calculations that run on the GPU.

## Question 4

*Why is it common to use an index buffer to specify which vertices should be connected into triangles, as opposed to relying on the order in which the vertices are specified in the vertex buffer(s)?*

It allows to re-use the same vertex multiple times without having to copy the 3 coordinates, thus saving memory.

## Question 5

*While the last input of `gl::VertexAttribPointer()` is a pointer, we usually pass in a null pointer. Describe a situation in which you would pass a non-zero value into this function.*

If loading not just the coordinates into our buffer but also the UV coordinates we would need to tell OpenGL how many bytes it has to cross until a new set of numbers begins. Example: 3D coordinates + 2D UV coordinates results in $3 * 4 + 2 * 4 = 20$ Bytes.

# Task 2d

![](img/face-flipped.png)

![](img/sierpinski-flipped.png)

![](img/face-colored.png)

![](img/sierpinski-colored.png)


To achieve the flipping, we simply invert the X and Y components of our vertices and write the result back to `gl_Position`:

```glsl
gl_Position = vec4(position.x * -1.0f, position.y * -1.0f, position.z, 1.0f);
```

To recolor the mesh, we set the color output to our desired color. (yellow)

```glsl
in vec3 position;
void main()
{
    gl_Position = vec4(position.x * -1.0f, position.y * -1.0f, position.z, 1.0f);
}
```

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
This[^this_is_a_unique_footnote_label] is a footnote. <!-- defined below -->
This^[Footnotes can also be written inline] is also a footnote.


[This]: https://www.uio.no
[^this_is_a_unique_footnote_label]: In footnotes you can write anything tangentially related.

* This
* is
* a
* unordered
* list

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
block quote


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
