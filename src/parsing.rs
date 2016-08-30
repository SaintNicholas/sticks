use nom::{IResult, space, alphanumeric, multispace, be_f64, le_f64, digit};

use std::str;
use std::str::FromStr;

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

#[derive(Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

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

named!(parse_name<String>,
    chain!(
        tag!("newmtl") ~
        many0!(space) ~
        name: map_res!(alphanumeric, str::from_utf8) ~
        many0!(multispace),

        ||{name.to_string()}
    )
);

// named!(parse_color_ambient<Color>,
//     chain!(
//         tag!("Ka") ~
//         many0!(space) ~
//         red: le_f64 ~
//         many0!(space) ~
//         green: opt!(le_f64) ~
//         many0!(space) ~
//         blue: opt!(le_f64) ~
//         many0!(multispace),

//         ||{
//               let actual_green: f64 = match green {
//                 Some(i) => i,
//                 None => red
//               };
//               let actual_blue: f64 = match blue {
//                 Some(i) => i,
//                 None => red
//               };
//               Color{r: red, g: actual_green, b: actual_blue}}
//     )
// );

named!(parse_color_ambient<Color>,
    chain!(
        tag!("Ka") ~
        many0!(space) ~
        red: float64 ~
        many0!(space) ~
        green: opt!(float64) ~
        many0!(space) ~
        blue: opt!(float64) ~
        many0!(multispace),

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

named!(float64<f64>,
    chain!(
        a: map_res!(digit, str::from_utf8) ~
        b: opt!(tag!(".")) ~
        c: opt!(map_res!(digit, str::from_utf8)),

        ||{
               let mut float_string: String = a.to_string();
               if let Some(i) = c {
                  float_string = float_string + "." + &i;
               }
               // str::FromStr::from_str(float_string).unwrap()}
               f64::from_str(&float_string[..]).unwrap()}
    )
);

// named!(float64<f64>,
//     chain!(
//         f: float64str,

//         ||{str::FromStr::from_str(f).unwrap()}
//     )
// );

// named!(float64str<&str>,
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
fn test_parse3() {

  let test_case = &b"1.0"[..];

println!("{:?}", float64(test_case));
panic!();
}

#[test]
fn test_parse1() {

  let test_case = &b"Ka 0.123 13.244 1.0"[..];

println!("{:?}", parse_color_ambient(test_case));
panic!();
}

#[test]
fn test_parse2() {

  let test_case = &b"newmtl Material"[..];

println!("{:?}", parse_name(test_case));
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