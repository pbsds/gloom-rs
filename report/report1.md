---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Exercise 1
author:
  - Marius Arhaug
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

# Task 1

## Task 1 C)

A VAO has been instanitated with 5 distinc triangles. This can be shown in the screenshot provided. Each triangle is created by adding 3 vertexes into the vertices vector followed by the drawing direction for each vertex in the indices vector.

![](images/triangles.png){height=10em}

`\newpage`{=latex}

# Task 2

## Task 2 a)

Below is provided a picture of the triangle drawn by these vertices

<!-- prettier-ignore-->
`
\[
    v_0 = \begin{bmatrix}
           0.6 \\
           -0.8 \\
           -1.2 
         \end{bmatrix}
    v_1 = \begin{bmatrix}
           0 \\
           0.4 \\
           0
         \end{bmatrix}
    v_2 = \begin{bmatrix}
           -0.8 \\
           -0.2 \\
           1.2
         \end{bmatrix}
\]
`{=latex}

i) The name of this phenonom is called "clipping"

ii) It happens because the edges of the triangle appears to be going past the z-axis's visible values. And since the camera does not move it does not show the remainder of the triangle. i.e the two edges that go past the max visible z values (either negative or positive)

iii) Its purpose is to reduce the number of polygons for the program to render. The program then only needs to draw the polygon that are visible from the camera's perspective.

![](images/triangle2a.png){height=10em margin=auto }

## Task 2 b)

i) When chaning the order of vertices drawn by modifying the values in the **index buffer** the triangle disapears.

ii) It happens because of face culling. It is a process where triangles can be disgarded and not render due to their "facing". During primitive assembly the order of vertices in a triangle has a visual orientation, that is used to determine whether the triangle is seen from the front or the back. Triangles seen from the back are usually not seen by the camera, and therefore does not warant the program to render them.

iii) The effect is dependent on the order in which vertices are being drawn. If they are drawn in a clockwise or counter clockwise fashion.

The function

```rust
fn gl::FrontFace(mode : types::GLenum)

```

may change wether clockwise or counter-clockwise winded triangles are labeled as front or back facing. By default counter-clockwise triangles are labeled as frontfacing. So by swapping the order in the **index buffer** wer changed the winding of our triangle from counter-clockwise to clockwise.

## Task 2 c)

i) The depth buffer is refreshed because it is used to compare z-values of fragments in the scene. If the buffer is not cleared it will compare new objects' z value against the previous frame, which does not make sense.

ii)

iii) The two most commonly used shaders are **fragment** and **vertex** shader

- The vertex shader is responsible for projection. This projection can be used for example to give perspective to objects.

- The fragment shader is responsible for coloring of each fragment. Fragments are pixels that OpenGL draws.

iv) The IBO index is used so that in the case where a vertex is used for multiple triangles, it does not need to be entered multiple times within the VBO. Instead the IBO's indices can be used to refer which vertexes in the VBO to draw from and to.

v) The `pointer` in the

```rust
fn gl::VertexAttribPointer(...)
```

is used to define an offset for the number of bytes until the first value in the buffer. If the buffer is of only one type we can set this value to 0.

## Task 2 d)

The simple.vert vertex shader file is updated to include a uniform value `mvp` which is a 3x3 matrix. This matrix is then used to perform a composite transformation to enable both a vertical and horizontal flip. The value of the matrix `mvp` is set after the `gl::UseProgram` is used by the shader struct in `shader.rs`

The composite transofrmation is made up of two affine transformations:

- To rotate the positions sent to the shader vertically we only have to provide a negative idenity matrix.

- To rortate the horizontal we may rotate the triangle along the z-axis. This can be done using the `get_rotation_matrix(theta, dir)` function, stored in the util.rs file.

To save computation these two matrixes are then multipled before the final matrix multiplication.

Here is the resulting image

![Rotated and recolored](./images/rotated_red_triangle.png)
