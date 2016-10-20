#[macro_use]
extern crate nom;
extern crate clap;

mod geometry;
mod wavefront;

use geometry::matrix::*;
use geometry::vector::*;
use clap::{App, Arg, SubCommand};
use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use wavefront::object_parser::{parse_object, Object};

fn computePixelCoordinates (
    pWorld: &Vec3<f64>,
    worldToCamera: &Matrix44<f64>,
    canvasWidth: f64,
    canvasHeight: f64,
    imageWidth: u32,
    imageHeight: u32) -> (u32, u32) {
    
    /* Take the point in the world coordinate system and and translate
       it into the camera coordinate system. */
    let pCamera = worldToCamera.multVecMatrix(pWorld);

    /* Assuming the distance between the camera and the image plane is 1,
       project this point onto the image plane (the screen), (to the
       screen coordinate system). */
    let pScreenX = pCamera.x / -pCamera.z;
    let pScreenY = pCamera.y / -pCamera.z;

    /* Convert the point in the screen coordinate system into a NDC coordinate system. */
    let pNDCX = (pScreenX + canvasWidth * 0.5) / canvasWidth;
    let pNDCY = (pScreenY + canvasHeight * 0.5) / canvasHeight;

    /* Convert the point in the NCD coordinate system into the raster coordinate system. */
    let pRasterX: u32 = (pNDCX * imageWidth as f64) as u32;
    let pRasterY: u32 = ((1.0 - pNDCY) * imageHeight as f64) as u32;

    (pRasterX, pRasterY)
}

fn main() {
    let mut object: Option<Object> = None;
    let mut output: Option<String> = None;

    let matches = App::new("sticks")
        .version("0.0.1")
        .author("Nick Goote <ngoote@gmail.com>")
        .about("Renders a wireframe model of the xtree object.")
        .arg(Arg::with_name("material")
            .short("m")
            .long("material")
            .value_name("FILE")
            .help("Sets a material file")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("object")
            .short("j")
            .long("object")
            .value_name("FILE")
            .help("Sets the object file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o`")
            .long("output")
            .value_name("FILE")
            .help("Sets the output file")
            .takes_value(true)
            .required(true))
        .get_matches();

    if let Some(o) = matches.value_of("output") {
        output = Some(o.to_string());
    } else {
        // This shouldn't happen because this parameter
        // is marked required.
        panic!("Failed to supply output file.")
    }

    if let Some(j) = matches.value_of("object") {
        let mut file = match File::open(j) {
            Err(why) => panic!("couldn't open {}: {}", j, why.description()),
            Ok(file) => file,
        };

        let mut s = String::new();
        let _ = match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read from {}: {}", j, why.description()),
            _ => ()
        };

        object = match parse_object(&s) {
            Err(why) => panic!("{}", why),
            Ok(obj) => Some(obj),
        };
    }  else {
        // This shouldn't happen because this parameter
        // is marked required.
        panic!("Failed to supply object file.")
    }

    let mut output_file = match File::create(output.unwrap()) {
        Err(why) => panic!("couldn't open {}: {}", "File to write", why.description()),
        Ok(output_file) => output_file,
    };

    output_file.write_all(b"<svg version=\"1.1\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" xmlns=\"http://www.w3.org/2000/svg\" height=\"512\" width=\"512\">");

    let cameraToWorld: Matrix44<f64> = Matrix44::new(0.871214, 0.0, -0.490904, 0.0, -0.192902, 0.919559, -0.342346, 0.0, 0.451415, 0.392953, 0.801132, 0.0, 14.777467, 29.361945, 27.993464, 1.0);
    let worldToCamera: Matrix44<f64> = cameraToWorld.clone().inverse();
    let canvasWidth: f64 = 2.0;
    let canvasHeight: f64 = 2.0;
    let imageWidth: u32 = 512;
    let imageHeight: u32 = 512;
    let uobject = object.unwrap();

    for i in 0..uobject.triangles.len() {
        let v0Worldy = uobject.triangles[i].v1;
        let ref v0World: Vec3<f64> = Vec3::new(v0Worldy.x, v0Worldy.y, v0Worldy.z);
        let v1Worldy = uobject.triangles[i].v2;
        let ref v1World: Vec3<f64> = Vec3::new(v1Worldy.x, v1Worldy.y, v1Worldy.z);
        let v2Worldy = uobject.triangles[i].v3;
        let ref v2World: Vec3<f64> = Vec3::new(v2Worldy.x, v2Worldy.y, v2Worldy.z);

        let (v0RasterX, v0RasterY) = computePixelCoordinates(v0World, &worldToCamera, canvasWidth, canvasHeight, imageWidth, imageHeight);
        let (v1RasterX, v1RasterY) = computePixelCoordinates(v1World, &worldToCamera, canvasWidth, canvasHeight, imageWidth, imageHeight);
        let (v2RasterX, v2RasterY) = computePixelCoordinates(v2World, &worldToCamera, canvasWidth, canvasHeight, imageWidth, imageHeight);

        output_file.write_fmt(format_args!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(0,0,0);stroke-width:1\" />", v0RasterX, v0RasterY, v1RasterX, v1RasterY));
        output_file.write_fmt(format_args!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(0,0,0);stroke-width:1\" />", v1RasterX, v1RasterY, v2RasterX, v2RasterY));
        output_file.write_fmt(format_args!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:rgb(0,0,0);stroke-width:1\" />", v2RasterX, v2RasterY, v0RasterX, v0RasterY));
    }

    output_file.write_all(b"</svg>");
}