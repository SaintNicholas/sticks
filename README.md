# sticks
Triangle mesh wireframe renderer written in Rust

## Overview

Sticks is a triangle mesh wireframe renderer, based on [Scratchpixel's guide to Computing the Pixel Coordinates of a 3D Point](http://www.scratchapixel.com/lessons/3d-basic-rendering/computing-pixel-coordinates-of-3d-point).

So far, it can render a single set of vertices and triangles that are hardcoded within the source code, to an svg file.

<img src="http://i.imgur.com/rjkEVTW.png" width="256">

There are a number of improvements that I plan on making:
- Have the executable produces take in command line arguments for the Wavefront .obj file as input, and the .svg file as output.
- Be able to parse a Wavefront .obj file to get a list of vertices and triangles. (triangles only, no other shapes).
- Be able to output to a .svg file.
- Make the Vector and Matrix types more generic (have macros do most of the heavy lifting, and make the macros dimension agnostic).
