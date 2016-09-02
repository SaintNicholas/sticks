use nom::{space, alphanumeric, digit, eof, not_line_ending};

use std::str;
use std::str::FromStr;
use std::collections::HashSet;
use nom::IResult::*;

/* Can I remove the String and make this Copy? */
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    if let Done(_, parsed) = parse_material_values(string.as_bytes()) {
        construct_material_structs(parsed)
    } else {
        Err(format!("Parser Error: {}", string))
    }
}

fn construct_material_structs(values: Vec<Value>) -> Result<Vec<Material>, String> {
    let mut materials: Vec<Material> = Vec::new();
    let mut error_string = String::new();

    let mut last_name_pos = 0;
    'constructing_materials: for i in 0..values.len() + 1 {
        let mut process_because_found_name = false;

        if let Value::Name(_) = values[i] {
            process_because_found_name = true;
        }

        if i == values.len() + 1 {
            process_because_found_name = true;
        }

        if process_because_found_name && i != 0 {
            last_name_pos = i;
            let construction_result = construct_material_struct(&values[last_name_pos..i]);
            match construction_result {
                Ok(material) => {
                    materials.push(material);
                },
                Err(string) => {
                    error_string = string;
                    break 'constructing_materials;
                },
            }
        }
    }

    if error_string.is_empty() {
        Ok(materials)
    } else {
        Err(error_string)
    }
}

fn construct_material_struct(values: &[Value]) -> Result<Material, String> {
    let mut material: Material = Default::default();
    let mut found_name = false;
    let mut found_ambient = false;
    let mut found_diffuse = false;
    let mut found_specular = false;
    let mut error_strings = HashSet::new();

    'parsing_values: for value in values {
        match value {
            &Value::Name(ref name) => {
                if false == found_name {
                    found_name = true;
                    material.name = name.clone();
                } else {
                    error_strings.insert("Duplicate names found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorAmbient(ref color) => {
                if false == found_name {
                    found_ambient = true;
                    material.color_ambient = *color;
                } else {
                    error_strings.insert("Duplicate ambient colors found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorDiffuse(ref color) => {
                if false == found_name {
                    found_diffuse = true;
                    material.color_diffuse = *color;
                } else {
                    error_strings.insert("Duplicate diffuse colors found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorSpecular(ref color) => {
                if false == found_name {
                    found_specular = true;
                    material.color_specular = *color;
                } else {
                    error_strings.insert("Duplicate specular colors found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorTransmission(ref color) => {
                if let Some(_) = material.color_transmission {
                    error_strings.insert("Duplicate transmission colors found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.color_transmission = Some(*color);
                }
            },
            &Value::Illum(ref illum) => {
                if let Some(_) = material.illumination {
                    error_strings.insert("Duplicate illumination found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.illumination = Some(*illum);
                }
            },
            &Value::Alpha(ref alpha) => {
                if let Some(_) = material.alpha {
                    error_strings.insert("Duplicate alpha found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.alpha = Some(*alpha);
                }
            },
            &Value::SpecularCoefficient(ref coefficient) => {
                if let Some(_) = material.specular_coefficient {
                    error_strings.insert("Duplicate specular coefficient found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.specular_coefficient = Some(*coefficient);
                }
            },
            &Value::OpticalDensity(ref density) => {
                if let Some(_) = material.optical_density {
                    error_strings.insert("Duplicate optical density found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.optical_density = Some(*density);
                }
            },
        }
    }

    if false == found_name {
        error_strings.insert("Name not found while constructing Material. It is a necessary field.".to_string());
    }
    if false == found_ambient {
        error_strings.insert("Ambient color not found while constructing Material. It is a necessary field.".to_string());
    }
    if false == found_diffuse {
        error_strings.insert("Diffuse color found while constructing Material. It is a necessary field.".to_string());
    }
    if false == found_specular {
        error_strings.insert("Specular color not found while constructing Material. It is a necessary field.".to_string());
    }

    if error_strings.is_empty() {
        Ok(material)
    } else {
        Err(error_strings.iter().fold(String::new(), |acc, ref x| acc + "\n" + x))
    }
}

named!(parse_material_values<Vec<Value> >,
    many0!(
        chain!(
            many0!(parse_ignored_line) ~
            value: alt!(
                parse_name_value |
                parse_color_ambient_value |
                parse_color_diffuse_value |
                parse_color_specular_value |
                parse_color_transmission_value |
                parse_illum_value |
                parse_alpha_value |
                parse_specular_coefficient_value |
                parse_optical_density_value
            ) ~
            many0!(parse_ignored_line),

            ||{value}
        )
    )
);

named!(parse_name_value<Value>,
    chain!(
        name: parse_name,

        ||{Value::Name(name)}
    )
);

named!(parse_name<String>,
    chain!(
        many0!(space) ~
        tag!("newmtl") ~
        many0!(space) ~
        name: map_res!(alphanumeric, str::from_utf8) ~
        parse_ignored_line,

        ||{name.to_string()}
    )
);

named!(parse_color_ambient_value<Value>,
    chain!(
        color: parse_color_ambient,

        ||{Value::ColorAmbient(color)}
    )
);

named!(parse_color_ambient<Color>,
    chain!(
        many0!(space) ~
        tag!("Ka") ~
        color: parse_color ~
        parse_ignored_line,

        ||{color}
    )
);

named!(parse_color_diffuse_value<Value>,
    chain!(
        color: parse_color_diffuse,

        ||{Value::ColorDiffuse(color)}
    )
);

named!(parse_color_diffuse<Color>,
    chain!(
        many0!(space) ~
        tag!("Kd") ~
        color: parse_color ~
        parse_ignored_line,

        ||{color}
    )
);

named!(parse_color_specular_value<Value>,
    chain!(
        color: parse_color_specular,

        ||{Value::ColorSpecular(color)}
    )
);

named!(parse_color_specular<Color>,
    chain!(
        many0!(space) ~
        tag!("Ks") ~
        color: parse_color ~
        parse_ignored_line,

        ||{color}
    )
);

named!(parse_color_transmission_value<Value>,
    chain!(
        color: parse_color_transmission,

        ||{Value::ColorTransmission(color)}
    )
);

named!(parse_color_transmission<Color>,
    chain!(
        many0!(space) ~
        tag!("Ts") ~
        color: parse_color ~
        parse_ignored_line,

        ||{color}
    )
);

named!(parse_color<Color>,
    chain!(
        many0!(space) ~
        red: parse_f64 ~
        many0!(space) ~
        green: opt!(parse_f64) ~
        many0!(space) ~
        blue: opt!(parse_f64),

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

named!(parse_illum_value<Value>,
    chain!(
        illum: parse_illum,

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

named!(parse_illum<usize>,
    chain!(
        many0!(space) ~
        tag!("illum") ~
        many0!(space) ~
        illum: parse_usize ~
        parse_ignored_line,

        ||{illum}
    )
);

named!(parse_alpha_value<Value>,
    chain!(
        alpha: parse_alpha,

        ||{Value::Alpha(alpha)}
    )
);

named!(parse_alpha<f64>,
    chain!(
        many0!(space) ~
        tag!("d") ~
        many0!(space) ~
        alpha: parse_f64 ~
        many0!(space) ~
        parse_ignored_line,

        ||{alpha}
    )
);

named!(parse_specular_coefficient_value<Value>,
    chain!(
        specular_coefficient: parse_specular_coefficient,

        ||{Value::SpecularCoefficient(specular_coefficient)}
    )
);

named!(parse_specular_coefficient<f64>,
    chain!(
        many0!(space) ~
        tag!("Ns") ~
        many0!(space) ~
        alpha: parse_f64 ~
        many0!(space) ~
        parse_ignored_line,

        ||{alpha}
    )
);

named!(parse_optical_density_value<Value>,
    chain!(
        optical_density: parse_optical_density,

        ||{Value::SpecularCoefficient(optical_density)}
    )
);

named!(parse_optical_density<f64>,
    chain!(
        many0!(space) ~
        tag!("Ni") ~
        many0!(space) ~
        alpha: parse_f64 ~
        many0!(space) ~
        parse_ignored_line,

        ||{alpha}
    )
);

named!(parse_f64<f64>,
    chain!(
        a: map_res!(digit, str::from_utf8) ~
        opt!(tag!(".")) ~
        c: opt!(map_res!(digit, str::from_utf8)),

        ||{
               let mut float_string: String = a.to_string();
               if let Some(i) = c {
                  float_string = float_string + "." + &i;
               }
               f64::from_str(&float_string[..]).unwrap()}
    )
);

named!(parse_usize<usize>,
    chain!(
    a: map_res!(digit, str::from_utf8),
    ||{
           let usize_string: String = a.to_string();
           usize::from_str(&usize_string[..]).unwrap()}
    )
);

named!(parse_ignored_line,
    chain!(
        alt!(parse_blank_line | parse_comment),

        || { &b""[..] }
    )
);

named!(parse_blank_line,
    chain!(
        many0!(space) ~
        alt!(eof | parse_eol),
        
        || { &b""[..] }
    )
);

named!(parse_comment,
    chain!(
        many0!(space) ~
        tag!("#") ~
        not_line_ending ~
        alt!(eof | parse_eol),
        
        || { &b""[..] }
    )
);

named!(parse_eol,
    alt!(tag!("\n") | tag!("\r\n") | tag!("\u{2028}") | tag!("\u{2029}"))
);

#[cfg(test)]
mod tests
{
    use nom::IResult::*;
    use super::{Value, parse_name_value};

    #[test]
    fn parse_name_value_should_parse_properly() {
        let test_case_1 = &b" newmtl materialname3
"[..];

        println!("{:?}", parse_name_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::Name("materialname3".to_string())), parse_name_value(test_case_1));
    }







    // #[test]
    // fn test_parse4() {

    //     let test_case = &b" # TEst

    // newmtl material1
    // Ka 1.1 2.2 3.3 # haha
    // # oh oh oh


    // # hehehe
    // Ka 2.2 3.3 4.4


    // "[..];

    //     println!("{:?}", parse_material_values(test_case));
    //     panic!();
    // }

    // #[test]
    // fn test_parse3() {

    //   let test_case = &b"1.0"[..];

    // println!("{:?}", f64(test_case));
    // panic!();
    // }

    // #[test]
    // fn test_parse2() {

    //   let test_case = &b"Ka 0.123 13.244 1.0"[..];

    // println!("{:?}", parse_color_ambient_value(test_case));
    // panic!();
    // }

    // #[test]
    // fn test_parse1() {

    //   let test_case = &b"newmtl Material"[..];

    // println!("{:?}", parse_name_value(test_case));
    // panic!();

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
    // }
}