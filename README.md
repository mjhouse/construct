# Introduction

This project is an attempt to build residential design software that can A) automatically verify that the building conforms to 
regulations for the location in which it is built and B) be able to generate a bill of materials (BOM) for the construction. There 
are secondary features that could be pursued as well- see the goals section for those.

# Goals

## Primary

* Automatically validate the design from a regulatory point of view
* Generate a bill-of-materials purchasing list

## Secondary

* Validate the geometry of the design
* Solve constraints and connections between parts
* Fetch part/price data from some public API for BOM
* Provide GUI/3d view

# Overview

This project will initially provide a library of common construction objects. Each type of object will be associated with 
a material, geometric data, connection points and so on. At a low level, these parts have geometry and will simply exist 
in a shared 3d space. Several layers of constraint solvers will operate on this space to validate the proposed design. 

From bottom to top:

1.  A geometry constraint solver will verify that no parts intersect, or only intersect following specific transformation
    rules. For example, a 2x4 may "intersect" due to joining at an angle, but an angle at the point of intersection is 
    acceptable because in the real structure the wood can be cut to fit the angle.

2.  A connection constraint solver will inspect positioning of parts to determine whether all parts are joined at valid 
    connection points and issue warnings for freestanding parts.

3.  A design constraint solver will apply a set of design requirements for the region in which the building is going 
    to be constructed. For example, states have regulations about the widths of doors and windows, the distance between 
    upright 2x4s etc.

Each part will have the following properties:

* **Name**:        The name of the object (e.g. "2x4")
* **Geometry**:    Faces and vertices of the real geometry
* **Bounds**:      A bounding box that tightly encloses the real geometry (calculated)
* **Attributes**:  Part-specific attributes (e.g. Length, Width)
* **Connections**: A vertex and radius for a connection point on the part 
* **Metadata**:    General labels, notes etc.

It's worth noting here that the attributes of the part should transform the geometry as they are modified. So if a 2x4 has 
a "Length" attribute, then modifying that attribute will change the real geometry. Attributes will have a collection of 
(range,transform,multiple) tuples. When the value or values of the attribute (length, width or size etc.) is changed, the 
attribute will multiply the input value by each multiple, pass the multiple into the transformations, and apply the transformations
to each vertex in the ranges.

All parts should be able to serialize their geometry to an obj file. Non-geometry related features and attributes should use 
magic comments in the output obj file, but the non-comment information should be a (subset of) the standard obj format.

# Solvers

## Regulatory Solver

Building codes are generally published and sold by state (as an example - [Alabama](https://dcm.alabama.gov/bldg_code.aspx#BC)). 
I'll need to buy one of the books and read it to figure out how to turn it into some sort of structured set of rules. As the project 
progresses, I'll have to acquire more. The rules should be in some external file format (not hard coded) so that you can swap them 
out by year or state pretty easily.

May be able to avoid buying books: https://up.codes/codes/tennessee

## Connection Solver

TODO

## Geometric Solver

TODO
