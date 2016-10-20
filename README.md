# sticks
Triangle mesh wireframe renderer written in Rust

## Overview

Sticks is a triangle mesh wireframe renderer, based on [Scratchpixel's guide to Computing the Pixel Coordinates of a 3D Point](http://www.scratchapixel.com/lessons/3d-basic-rendering/computing-pixel-coordinates-of-3d-point).

It can parse an object & set of material files that are specified, render the wireframe of that object, and write the output to an svg file. It currently only works with the xtree object file given by Scratchpixel because the world coordinate matrix was given by scratchpixel, and it only works for their specific example.

<img src="http://i.imgur.com/rjkEVTW.png" width="256">
