use nom::{IResult, space, alphanumeric, multispace, digit, eof, not_line_ending};

use std::str;
use std::str::FromStr;
use nom::IResult::*;

#[derive(Debug)]
pub struct Material {
    pub name: String,                       // newmtl
    pub color_ambient: Color,               // Ka
    pub color_diffuse: Color,               // Kd
    pub color_specular: Color,              // Ks
    pub color_transmission: Option<Color>,  // Tf
    pub illumination: Option<Illumination>, // illum
    pub alpha: Option<f64>,                 // d
    pub specular_coefficient: Option<f64>,  // Ns
    pub optical_density: Option<f64>,       // Ni
}

impl Default for Material {
    fn default() -> Material {
        Material{
            name: String::new(),
            color_ambient: Default::default(),
            color_diffuse: Default::default(),
            color_specular: Default::default(),
            color_transmission: None,
            illumination: None,
            alpha: None,
            specular_coefficient: None,
            optical_density: None,
        }
    }
}

#[derive(Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Default for Color {
    fn default() -> Color {
        Color{r: 0.0, g: 0.0, b: 0.0}
    }
}

#[derive(Debug)]
pub enum Illumination {
  ColorOnAmbientOff,
  ColorOnAmbientOn,
  HighlightOn,
  ReflectionOnAndRayTraceOn,
  TransparencyGlassOnReflectionRayTraceOn,
  ReflectionFresnelOnAndRayTraceOn,
  TransparencyRefractionOnReflectionFresnelOffAndRayTraceOn,
  TransparencyRefractionOnReflectionFresnelOnAndRayTraceOn,
  TeflectionOnAndRayTraceOff,
  TransparencyGlassOnReflectionRayTraceOff,
  CastsShadowsOntoInvisibleSurfaces,
}

#[derive(Debug)]
enum Value {
    Name(String),
    ColorAmbient(Color),
    ColorDiffuse(Color),
    ColorSpecular(Color),
    ColorTransmission(Color),
    Illum(Illumination),
    Alpha(f64),
    SpecularCoefficient(f64),
    OpticalDensity(f64),
}

pub fn parse_materials(string: &str) -> Result<Vec<Material>, String> {
    if let Done(_, parsed) = material_values(string.as_bytes()) {
        construct_material_structs(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}

fn construct_material_structs(values: Vec<Value>) -> Result<Vec<Material>, String> {
    let mut materials: Vec<Material> = Vec::new();

    let mut last_name_pos = 0;
    for i in 0..values.len() + 1 {
        if let Value::Name(_) = values[i] {
            let material = construct_material_struct(&values[last_name_pos..i]);
            last_name_pos = i
        }
    }

    let material = construct_material_struct(&values[last_name_pos..values.len()+1]);

    Ok(materials)
}

fn construct_material_struct(values: &[Value]) -> Result<Material, String> {
    let mut material: Material = Default::default();
    for value in values {
        match value {
            _ => println!("Found one")
        }
    }
    return Err("Fail".to_string())
}

named!(material_values<Vec<Value> >,
    many0!(
        chain!(
            many0!(ignored_line) ~
            value: alt!(
                name_value |
                color_ambient_value |
                color_diffuse_value |
                color_specular_value |
                color_transmission_value |
                illum_value |
                alpha_value |
                specular_coefficient_value |
                optical_density_value
            ) ~
            many0!(ignored_line),

            ||{value}
        )
    )
);

named!(name_value<Value>,
    chain!(
        name: name,

        ||{Value::Name(name)}
    )
);

named!(name<String>,
    chain!(
        many0!(space) ~
        tag!("newmtl") ~
        many0!(space) ~
        name: map_res!(alphanumeric, str::from_utf8) ~
        ignored_line,

        ||{name.to_string()}
    )
);

named!(color_ambient_value<Value>,
    chain!(
        color: color_ambient,

        ||{Value::ColorAmbient(color)}
    )
);

named!(color_ambient<Color>,
    chain!(
        many0!(space) ~
        tag!("Ka") ~
        color: color ~
        ignored_line,

        ||{color}
    )
);

named!(color_diffuse_value<Value>,
    chain!(
        color: color_diffuse,

        ||{Value::ColorDiffuse(color)}
    )
);

named!(color_diffuse<Color>,
    chain!(
        many0!(space) ~
        tag!("Kd") ~
        color: color ~
        ignored_line,

        ||{color}
    )
);

named!(color_specular_value<Value>,
    chain!(
        color: color_specular,

        ||{Value::ColorSpecular(color)}
    )
);

named!(color_specular<Color>,
    chain!(
        many0!(space) ~
        tag!("Ks") ~
        color: color ~
        ignored_line,

        ||{color}
    )
);

named!(color_transmission_value<Value>,
    chain!(
        color: color_transmission,

        ||{Value::ColorTransmission(color)}
    )
);

named!(color_transmission<Color>,
    chain!(
        many0!(space) ~
        tag!("Ts") ~
        color: color ~
        ignored_line,

        ||{color}
    )
);

named!(color<Color>,
    chain!(
        many0!(space) ~
        red: f64 ~
        many0!(space) ~
        green: opt!(f64) ~
        many0!(space) ~
        blue: opt!(f64),

        ||{
              let actual_green: f64 = match green {
                Some(i) => i,
                None => red
              };
              let actual_blue: f64 = match blue {
                Some(i) => i,
                None => red
              };
              Color{r: red, g: actual_green, b: actual_blue}}
    )
);

named!(illum_value<Value>,
    chain!(
        illum: illum,

        ||{
              let illum_type: Illumination = match illum {
                  0 => Illumination::ColorOnAmbientOff,
                  1 => Illumination::ColorOnAmbientOn,
                  2 => Illumination::HighlightOn,
                  3 => Illumination::ReflectionOnAndRayTraceOn,
                  4 => Illumination::TransparencyGlassOnReflectionRayTraceOn,
                  5 => Illumination::ReflectionFresnelOnAndRayTraceOn,
                  6 => Illumination::TransparencyRefractionOnReflectionFresnelOffAndRayTraceOn,
                  7 => Illumination::TransparencyRefractionOnReflectionFresnelOnAndRayTraceOn,
                  8 => Illumination::TeflectionOnAndRayTraceOff,
                  9 => Illumination::TransparencyGlassOnReflectionRayTraceOff,
                  10 => Illumination::CastsShadowsOntoInvisibleSurfaces,
                  _ => panic!("Invalid illum")
              };
              Value::Illum(illum_type)}
    )
);

named!(illum<usize>,
    chain!(
        many0!(space) ~
        tag!("illum") ~
        many0!(space) ~
        illum: usize ~
        ignored_line,

        ||{illum}
    )
);

named!(alpha_value<Value>,
    chain!(
        alpha: alpha,

        ||{Value::Alpha(alpha)}
    )
);

named!(alpha<f64>,
    chain!(
        many0!(space) ~
        tag!("d") ~
        many0!(space) ~
        alpha: f64 ~
        many0!(space) ~
        ignored_line,

        ||{alpha}
    )
);

named!(specular_coefficient_value<Value>,
    chain!(
        specular_coefficient: specular_coefficient,

        ||{Value::SpecularCoefficient(specular_coefficient)}
    )
);

named!(specular_coefficient<f64>,
    chain!(
        many0!(space) ~
        tag!("Ns") ~
        many0!(space) ~
        alpha: f64 ~
        many0!(space) ~
        ignored_line,

        ||{alpha}
    )
);

named!(optical_density_value<Value>,
    chain!(
        optical_density: optical_density,

        ||{Value::SpecularCoefficient(optical_density)}
    )
);

named!(optical_density<f64>,
    chain!(
        many0!(space) ~
        tag!("Ni") ~
        many0!(space) ~
        alpha: f64 ~
        many0!(space) ~
        ignored_line,

        ||{alpha}
    )
);

named!(f64<f64>,
    chain!(
        a: map_res!(digit, str::from_utf8) ~
        b: opt!(tag!(".")) ~
        c: opt!(map_res!(digit, str::from_utf8)),

        ||{
               let mut float_string: String = a.to_string();
               if let Some(i) = c {
                  float_string = float_string + "." + &i;
               }
               f64::from_str(&float_string[..]).unwrap()}
    )
);

named!(usize<usize>,
    chain!(
    a: map_res!(digit, str::from_utf8),
    ||{
           let mut usize_string: String = a.to_string();
           usize::from_str(&usize_string[..]).unwrap()}
    )
);

named!(ignored_line,
    chain!(
        alt!(blank_line | comment),

        || { &b""[..] }
    )
);

named!(blank_line,
    chain!(
        many0!(space) ~
        alt!(eof | eol),
        
        || { &b""[..] }
    )
);

named!(comment,
    chain!(
        many0!(space) ~
        tag!("#") ~
        not_line_ending ~
        alt!(eof | eol),
        
        || { &b""[..] }
    )
);

named!(eol,
    alt!(tag!("\n") | tag!("\r\n") | tag!("\u{2028}") | tag!("\u{2029}"))
);

// named!(f64<f64>,
//     chain!(
//         f: f64str,

//         ||{str::FromStr::from_str(f).unwrap()}
//     )
// );

// named!(f64str<&str>,
//     many1!(alt!(digit | char!('.')))
// );

// named!(material_aggregator<&[u8], (&str, Material)>,
//     chain!(
//         tag!("newmtl") ~
//         space          ~
//         key: map_res!(alphanumeric, str::from_utf8)
//         ,
//         ||{("newmtl", Material{color_illumination: Color_Illumination{}})}
//         )
// );

// named!(materials_aggregator<&[u8], Vec<(&str, Material)> >, many0!(material_aggregator));

// fn materials(input: &[u8]) -> IResult<&[u8], HashMap<&str, Material> > {
//     let materials: HashMap<&str, Material> = HashMap::new();

//     materials
// }



// newmtl my_mtl
// 
// Ka 0.0435 0.0435 0.0435 
//   -> Ka r g b # g and b are optional, if not specified, equal to r. Normally in range 0.0 to 1.0. Values outside range increase or decrease the stat accordingly.
// Kd 0.1086 0.1086 0.1086 
//   -> Kd r g b # g and b are optional, if not specified, equal to r. Normally in range 0.0 to 1.0. Values outside range increase or decrease the stat accordingly.
// Ks 0.0000 0.0000 0.0000 
//   -> Ks r g b # g and b are optional, if not specified, equal to r. Normally in range 0.0 to 1.0. Values outside range increase or decrease the stat accordingly.
// Tf 0.9885 0.9885 0.9885 
//   -> Tf r g b # g and b are optional, if not specified, equal to r. Normally in range 0.0 to 1.0. Values outside range increase or decrease the stat accordingly.
// illum 6 
//   -> Number can be 0 - 10. Defines the illumination method.
// d -halo 0.6600 
// d 0.6600
//   -> Value is between 0.0 and 1.0. Can be 'd factor' or 'd -halo factor'.
// Ns 10.0000 
//   -> Normally from 0 to 1000.
// sharpness 60 
//   -> Number from 0 to 1000, default 60.
// Ni 1.19713 
//   -> 0.001 to 10
// 
// map_Ka -s 1 1 1 -o 0 0 0 -mm 0 1 chrome.mpc 
// map_Kd -s 1 1 1 -o 0 0 0 -mm 0 1 chrome.mpc 
// map_Ks -s 1 1 1 -o 0 0 0 -mm 0 1 chrome.mpc 
// map_Ns -s 1 1 1 -o 0 0 0 -mm 0 1 wisp.mps 
// map_d -s 1 1 1 -o 0 0 0 -mm 0 1 wisp.mps 
// disp -s 1 1 .5 wisp.mps 
// decal -s 1 1 1 -o 0 0 0 -mm 0 1 sand.mps 
// bump -s 1 1 1 -o 0 0 0 -bm 1 sand.mpb 
// 
// refl -type sphere -mm 0 1 clouds.mpc 




#[test]
fn test_parse4() {

    let test_case = &b" # TEst

newmtl material1
Ka 1.1 2.2 3.3 # haha
# oh oh oh


# hehehe
Ka 2.2 3.3 4.4


"[..];

    println!("{:?}", material_values(test_case));
    panic!();
}

#[test]
fn test_parse3() {

  let test_case = &b"1.0"[..];

println!("{:?}", f64(test_case));
panic!();
}

#[test]
fn test_parse2() {

  let test_case = &b"Ka 0.123 13.244 1.0"[..];

println!("{:?}", color_ambient_value(test_case));
panic!();
}

#[test]
fn test_parse1() {

  let test_case = &b"newmtl Material"[..];

println!("{:?}", name_value(test_case));
panic!();

// let test_case =
// r#"
// # Blender MTL File: 'None'
// # Material Count: 2
// # name
// newmtl Material
// # Phong specular coefficient
// Ns 96.078431
// # ambient color (weighted)
// Ka 0.000000 0.000000 0.000000
// # diffuse color (weighted)
// Kd 0.640000 0.640000 0.640000
// # dissolve factor (weighted)
// Ks 0.500000 0.500000 0.500000
// # emissive color (weighted)
// Ke 0.100000 0.100000 0.100000
// # optical density (refraction)
// Ni 1.000000
// # alpha
// d 1.000000
// # illumination: 0=ambient, 1=ambient+diffuse, 2=ambient+diffuse+specular
// illum 2
// newmtl None
// Ns 0
// # ambient
// Ka 0.000000 0.000000 0.000000
// # diffuse
// Kd 0.8 0.8 0.8
// # specular
// Ks 0.8 0.8 0.8
// d 1
// illum 2
// "#;

  // let expected =
  //   Ok(MtlSet {
  //     materials: vec!(
  //       Material {
  //         name: "Material".to_owned(),
  //         specular_coefficient: 96.078431,
  //         color_ambient:  Color { r: 0.0,  g: 0.0,  b: 0.0  },
  //         color_diffuse:  Color { r: 0.64, g: 0.64, b: 0.64 },
  //         color_specular: Color { r: 0.5,  g: 0.5,  b: 0.5  },
  //         color_emissive: Some(Color { r: 0.1,  g: 0.1,  b: 0.1  }),
  //         optical_density: Some(1.0),
  //         alpha: 1.0,
  //         illumination: AmbientDiffuseSpecular,
  //         uv_map: None,
  //       },
  //       Material {
  //         name: "None".to_owned(),
  //         specular_coefficient: 0.0,
  //         color_ambient:  Color { r: 0.0, g: 0.0, b: 0.0 },
  //         color_diffuse:  Color { r: 0.8, g: 0.8, b: 0.8 },
  //         color_specular: Color { r: 0.8, g: 0.8, b: 0.8 },
  //         color_emissive: None,
  //         optical_density: None,
  //         alpha: 1.0,
  //         illumination: AmbientDiffuseSpecular,
  //         uv_map: None,
  //       }
  //     )
  //   });

  // assert_eq!(parse(test_case.to_owned()), expected);
}