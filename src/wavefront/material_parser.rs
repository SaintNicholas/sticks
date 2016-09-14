use nom::{space, digit, eof, not_line_ending};
use nom::IResult::*;
use std::str;
use wavefront::parser_utilities::{parse_f64, parse_ignored_line, parse_blank_line, parse_comment, parse_eol, not_space};

/* Can I remove the String and make this Copy? */
#[derive(Debug, PartialEq)]
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

        if i == values.len() {
            process_because_found_name = true;
        } else if let Value::Name(_) = values[i] {
            process_because_found_name = true;
        }

        if process_because_found_name && i != 0 {
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
            last_name_pos = i;
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
    let mut error_strings = Vec::new();

    'parsing_values: for value in values {
        match value {
            &Value::Name(ref name) => {
                if false == found_name {
                    found_name = true;
                    material.name = name.clone();
                } else {
                    error_strings.push("Duplicate names found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorAmbient(ref color) => {
                if false == found_ambient {
                    found_ambient = true;
                    material.color_ambient = *color;
                } else {
                    error_strings.push("Duplicate ambient colors found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorDiffuse(ref color) => {
                if false == found_diffuse {
                    found_diffuse = true;
                    material.color_diffuse = *color;
                } else {
                    error_strings.push("Duplicate diffuse colors found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorSpecular(ref color) => {
                if false == found_specular {
                    found_specular = true;
                    material.color_specular = *color;
                } else {
                    error_strings.push("Duplicate specular colors found while constructing Material.".to_string());
                    break 'parsing_values;
                }
            },
            &Value::ColorTransmission(ref color) => {
                if let Some(_) = material.color_transmission {
                    error_strings.push("Duplicate transmission colors found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.color_transmission = Some(*color);
                }
            },
            &Value::Illum(ref illum) => {
                if let Some(_) = material.illumination {
                    error_strings.push("Duplicate illumination found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.illumination = Some(*illum);
                }
            },
            &Value::Alpha(ref alpha) => {
                if let Some(_) = material.alpha {
                    error_strings.push("Duplicate alpha found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.alpha = Some(*alpha);
                }
            },
            &Value::SpecularCoefficient(ref coefficient) => {
                if let Some(_) = material.specular_coefficient {
                    error_strings.push("Duplicate specular coefficient found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.specular_coefficient = Some(*coefficient);
                }
            },
            &Value::OpticalDensity(ref density) => {
                if let Some(_) = material.optical_density {
                    error_strings.push("Duplicate optical density found while constructing Material.".to_string());
                    break 'parsing_values;
                } else {
                    material.optical_density = Some(*density);
                }
            },
        }
    }

    if false == found_name {
        error_strings.push("Name not found while constructing Material. It is a necessary field.".to_string());
    }
    if false == found_ambient {
        error_strings.push("Ambient color not found while constructing Material. It is a necessary field.".to_string());
    }
    if false == found_diffuse {
        error_strings.push("Diffuse color not found while constructing Material. It is a necessary field.".to_string());
    }
    if false == found_specular {
        error_strings.push("Specular color not found while constructing Material. It is a necessary field.".to_string());
    }

    if error_strings.is_empty() {
        Ok(material)
    } else {
        Err(error_strings.iter().fold(String::new(), |acc, ref x| acc + x + "\n"))
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
        name: map_res!(not_space, str::from_utf8) ~
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
        tag!("Tf") ~
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
        illum: alt!(parse_illum_0 | parse_illum_1 | parse_illum_2 | parse_illum_3 | parse_illum_4 | parse_illum_5 | parse_illum_6 | parse_illum_7 | parse_illum_8 | parse_illum_9 | parse_illum_10),

        ||{illum}
    )
);

macro_rules! create_named_parse_illum_value {
    ($name:ident, $value:expr, $valuevalue:expr) => (
        named!($name<Value>,
            chain!(
                many0!(space) ~
                tag!("illum") ~
                many0!(space) ~
                tag!(stringify!($value)) ~
                parse_ignored_line,

                ||{Value::Illum($valuevalue)}
            )
        );
    )
}
create_named_parse_illum_value!(parse_illum_0, 0, Illumination::ColorOnAmbientOff);
create_named_parse_illum_value!(parse_illum_1, 1, Illumination::ColorOnAmbientOn);
create_named_parse_illum_value!(parse_illum_2, 2, Illumination::HighlightOn);
create_named_parse_illum_value!(parse_illum_3, 3, Illumination::ReflectionOnAndRayTraceOn);
create_named_parse_illum_value!(parse_illum_4, 4, Illumination::TransparencyGlassOnReflectionRayTraceOn);
create_named_parse_illum_value!(parse_illum_5, 5, Illumination::ReflectionFresnelOnAndRayTraceOn);
create_named_parse_illum_value!(parse_illum_6, 6, Illumination::TransparencyRefractionOnReflectionFresnelOffAndRayTraceOn);
create_named_parse_illum_value!(parse_illum_7, 7, Illumination::TransparencyRefractionOnReflectionFresnelOnAndRayTraceOn);
create_named_parse_illum_value!(parse_illum_8, 8, Illumination::TeflectionOnAndRayTraceOff);
create_named_parse_illum_value!(parse_illum_9, 9, Illumination::TransparencyGlassOnReflectionRayTraceOff);
create_named_parse_illum_value!(parse_illum_10, 10, Illumination::CastsShadowsOntoInvisibleSurfaces);

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

        ||{Value::OpticalDensity(optical_density)}
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

#[cfg(test)]
mod tests
{
    use nom::IResult::*;
    use super::{Material,
                Color,
                Illumination,
                Value,
                parse_materials,
                construct_material_structs,
                construct_material_struct,
                parse_name_value,
                parse_color_ambient_value,
                parse_color_diffuse_value,
                parse_color_specular_value,
                parse_color_transmission_value,
                parse_illum_value,
                parse_alpha_value,
                parse_specular_coefficient_value,
                parse_optical_density_value};

    macro_rules! assert_nom_error {
        ($expression:expr) => (
            if let Error(_) = $expression {
                assert!(true);
            } else {
                assert!(false);
            }
        )
    }

    #[test]
    fn test_parse_materials_should_parse_properly() {
        test_parse_materials_should_parse_multi_material_mtl_file();
        test_parse_materials_should_return_error_if_failed();
    }

    fn test_parse_materials_should_parse_multi_material_mtl_file() {
        let expected_materials: Vec<Material> = vec!(
            Material{
                name: "Material".to_string(),
                color_ambient: Color{r: 0.0, g: 0.0, b: 0.0},
                color_diffuse: Color{r: 0.64, g: 0.64, b: 0.64},
                color_specular: Color{r: 0.5, g: 0.5, b: 0.5},
                color_transmission: Some(Color{r: 0.3, g: 0.254444, b: 0.876554}),
                illumination: Some(Illumination::HighlightOn),
                alpha: Some(1.0),
                specular_coefficient: Some(96.078431),
                optical_density: Some(1.0),
            },
            Material{
                name: "Material2".to_string(),
                color_ambient: Color{r: 0.1, g: 0.1, b: 0.1},
                color_diffuse: Color{r: 0.8, g: 0.8, b: 0.8},
                color_specular: Color{r: 0.8, g: 0.8, b: 0.8},
                color_transmission: None,
                illumination: None,
                alpha: None,
                specular_coefficient: None,
                optical_density: None,
            },
            Material{
                name: "Material3".to_string(),
                color_ambient: Color{r: 0.1, g: 0.2, b: 0.3},
                color_diffuse: Color{r: 0.14, g: 0.24, b: 0.74},
                color_specular: Color{r: 0.1, g: 0.54, b: 0.3},
                color_transmission: Some(Color{r: 0.3, g: 0.254444, b: 0.576554}),
                illumination: Some(Illumination::ReflectionOnAndRayTraceOn),
                alpha: Some(1.2),
                specular_coefficient: Some(44.078431),
                optical_density: Some(2.2),
            },
        );

        let mtl_file = "
# Blender MTL File: 'None'
# Material Count: 2
# name
newmtl Material
# Phong specular coefficient
Ns 96.078431
# ambient color (weighted)
Ka 0.000000 0.000000 0.000000
# diffuse color (weighted)
Kd 0.640000 0.640000 0.640000
# dissolve factor (weighted)
Ks 0.500000 0.500000 0.500000
# transmission color (weighted)
Tf 0.300000 0.254444 0.876554
# optical density (refraction)
Ni 1.000000
# alpha
d 1.000000
# illumination
illum 2



newmtl Material2
# ambient
Ka 0.100000 0.100000 0.100000
# diffuse
Kd 0.8 0.8 0.8
# specular
Ks 0.8 0.8 0.8

newmtl Material3
# Phong specular coefficient
Ns 44.078431
# ambient color (weighted)
Ka 0.100000 0.200000 0.300000
# diffuse color (weighted)
Kd 0.140000 0.240000 0.740000
# dissolve factor (weighted)
Ks 0.100000 0.540000 0.300000
# transmission color (weighted)
Tf 0.300000 0.254444 0.576554
# optical density (refraction)
Ni 2.200000
# alpha
d 1.200000
# illumination
illum 3
";

        assert_eq!(Ok(expected_materials), parse_materials(mtl_file));
    }

    fn test_parse_materials_should_return_error_if_failed() {
        let mtl_file = "
# Blender MTL File: 'None'
# Material Count: 2
# name
newmtl Material
# ambient color (weighted)
# Ka 0.000000 0.000000 0.000000
# diffuse color (weighted)
Kd 0.640000 0.640000 0.640000
# dissolve factor (weighted)
Ks 0.500000 0.500000 0.500000
";

        assert_eq!(Err("Ambient color not found while constructing Material. It is a necessary field.\n".to_string()), parse_materials(mtl_file));
    }

    #[test]
    fn construct_material_structs_should_construct_properly() {
        construct_material_structs_should_parse_single_material();
        construct_material_structs_should_parse_multiple_materials();
        construct_material_structs_should_return_error_if_failed();
    }

    fn construct_material_structs_should_parse_single_material() {
        let mut materials: Vec<Material> = Vec::new();
        let material: Material = Material{
            name: "material1".to_string(),
            color_ambient: Color{r: 1.0, g: 2.0, b: 3.0},
            color_diffuse: Color{r: 2.0, g: 3.0, b: 4.0},
            color_specular: Color{r: 3.0, g: 4.0, b: 5.0},
            color_transmission: Some(Color{r: 4.0, g: 5.0, b: 6.0}),
            illumination: Some(Illumination::ColorOnAmbientOff),
            alpha: Some(1.1),
            specular_coefficient: Some(2.2),
            optical_density: Some(3.3),
        };
        materials.push(material);

        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));

        assert_eq!(Ok(materials), construct_material_structs(values));
    }

    fn construct_material_structs_should_parse_multiple_materials() {
        let mut materials: Vec<Material> = Vec::new();
        let material1: Material = Material{
            name: "material1".to_string(),
            color_ambient: Color{r: 1.0, g: 2.0, b: 3.0},
            color_diffuse: Color{r: 2.0, g: 3.0, b: 4.0},
            color_specular: Color{r: 3.0, g: 4.0, b: 5.0},
            color_transmission: Some(Color{r: 4.0, g: 5.0, b: 6.0}),
            illumination: Some(Illumination::ColorOnAmbientOff),
            alpha: Some(1.1),
            specular_coefficient: Some(2.2),
            optical_density: Some(3.3),
        };
        let material2: Material = Material{
            name: "material2".to_string(),
            color_ambient: Color{r: 1.1, g: 2.2, b: 3.3},
            color_diffuse: Color{r: 2.2, g: 3.3, b: 4.4},
            color_specular: Color{r: 3.3, g: 4.4, b: 5.5},
            color_transmission: None,
            illumination: None,
            alpha: None,
            specular_coefficient: None,
            optical_density: None,
        };
        let material3: Material = Material{
            name: "material3".to_string(),
            color_ambient: Color{r: 1.9, g: 2.9, b: 3.9},
            color_diffuse: Color{r: 2.8, g: 3.8, b: 4.8},
            color_specular: Color{r: 3.7, g: 4.7, b: 5.7},
            color_transmission: Some(Color{r: 4.6, g: 5.6, b: 6.6}),
            illumination: Some(Illumination::ColorOnAmbientOn),
            alpha: Some(4.5),
            specular_coefficient: Some(4.6),
            optical_density: Some(4.7),
        };
        materials.push(material1);
        materials.push(material2);
        materials.push(material3);

        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        values.push(Value::Name("material2".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.1, g: 2.2, b: 3.3}));
        values.push(Value::ColorDiffuse(Color{r: 2.2, g: 3.3, b: 4.4}));
        values.push(Value::ColorSpecular(Color{r: 3.3, g: 4.4, b: 5.5}));
        values.push(Value::Name("material3".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.9, g: 2.9, b: 3.9}));
        values.push(Value::ColorDiffuse(Color{r: 2.8, g: 3.8, b: 4.8}));
        values.push(Value::ColorSpecular(Color{r: 3.7, g: 4.7, b: 5.7}));
        values.push(Value::ColorTransmission(Color{r: 4.6, g: 5.6, b: 6.6}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOn));
        values.push(Value::Alpha(4.5));
        values.push(Value::SpecularCoefficient(4.6));
        values.push(Value::OpticalDensity(4.7));

        assert_eq!(Ok(materials), construct_material_structs(values));
    }

    fn construct_material_structs_should_return_error_if_failed() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));

        assert_eq!(Err("Ambient color not found while constructing Material. It is a necessary field.\n".to_string()), construct_material_structs(values));
    }

    #[test]
    fn construct_material_struct_should_construct_properly() {
        construct_material_struct_should_parse_complete_material();
        construct_material_struct_should_parse_minimal_material();
        construct_material_struct_should_fail_if_no_name();
        construct_material_struct_should_fail_if_no_ambient_color();
        construct_material_struct_should_fail_if_no_diffuse_color();
        construct_material_struct_should_fail_if_no_specular_color();
        construct_material_struct_should_fail_if_no_values();
        construct_material_struct_should_fail_if_duplicate_name();
        construct_material_struct_should_fail_if_duplicate_ambient_color();
        construct_material_struct_should_fail_if_duplicate_diffuse_color();
        construct_material_struct_should_fail_if_duplicate_specular_color();
        construct_material_struct_should_fail_if_duplicate_transmission_color();
        construct_material_struct_should_fail_if_duplicate_illumination();
        construct_material_struct_should_fail_if_duplicate_alpha();
        construct_material_struct_should_fail_if_duplicate_specular_coefficient();
        construct_material_struct_should_fail_if_duplicate_optical_density();
    }

    fn construct_material_struct_should_parse_complete_material() {
        let material: Material = Material{
            name: "material1".to_string(),
            color_ambient: Color{r: 1.0, g: 2.0, b: 3.0},
            color_diffuse: Color{r: 2.0, g: 3.0, b: 4.0},
            color_specular: Color{r: 3.0, g: 4.0, b: 5.0},
            color_transmission: Some(Color{r: 4.0, g: 5.0, b: 6.0}),
            illumination: Some(Illumination::ColorOnAmbientOff),
            alpha: Some(1.1),
            specular_coefficient: Some(2.2),
            optical_density: Some(3.3),
        };
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));

        assert_eq!(Ok(material), construct_material_struct(&values));
    }

    fn construct_material_struct_should_parse_minimal_material() {
        let material: Material = Material{
            name: "material1".to_string(),
            color_ambient: Color{r: 1.0, g: 2.0, b: 3.0},
            color_diffuse: Color{r: 2.0, g: 3.0, b: 4.0},
            color_specular: Color{r: 3.0, g: 4.0, b: 5.0},
            color_transmission: None,
            illumination: None,
            alpha: None,
            specular_coefficient: None,
            optical_density: None,
        };
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));

        assert_eq!(Ok(material), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_no_name() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));

        assert_eq!(Err("Name not found while constructing Material. It is a necessary field.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_no_ambient_color() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));

        assert_eq!(Err("Ambient color not found while constructing Material. It is a necessary field.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_no_diffuse_color() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));

        assert_eq!(Err("Diffuse color not found while constructing Material. It is a necessary field.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_no_specular_color() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));

        assert_eq!(Err("Specular color not found while constructing Material. It is a necessary field.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_no_values() {
        let values: Vec<Value> = Vec::new();

        assert_eq!(Err("Name not found while constructing Material. It is a necessary field.\nAmbient color not found while constructing Material. It is a necessary field.\nDiffuse color not found while constructing Material. It is a necessary field.\nSpecular color not found while constructing Material. It is a necessary field.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_name() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::Name("material2".to_string()));
        assert_eq!(Err("Duplicate names found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::Name("material3".to_string()));
        assert_eq!(Err("Duplicate names found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_ambient_color() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::ColorAmbient(Color{r: 2.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate ambient colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::ColorAmbient(Color{r: 3.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate ambient colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_diffuse_color() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate diffuse colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::ColorDiffuse(Color{r: 3.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate diffuse colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_specular_color() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::ColorSpecular(Color{r: 2.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate specular colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate specular colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_transmission_color() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::ColorTransmission(Color{r: 2.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate transmission colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::ColorTransmission(Color{r: 3.0, g: 2.0, b: 3.0}));
        assert_eq!(Err("Duplicate transmission colors found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_illumination() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::Illum(Illumination::ColorOnAmbientOn));
        assert_eq!(Err("Duplicate illumination found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::Illum(Illumination::HighlightOn));
        assert_eq!(Err("Duplicate illumination found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_alpha() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::Alpha(1.2));
        assert_eq!(Err("Duplicate alpha found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::Alpha(1.3));
        assert_eq!(Err("Duplicate alpha found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_specular_coefficient() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::SpecularCoefficient(2.3));
        assert_eq!(Err("Duplicate specular coefficient found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::SpecularCoefficient(2.4));
        assert_eq!(Err("Duplicate specular coefficient found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    fn construct_material_struct_should_fail_if_duplicate_optical_density() {
        let mut values: Vec<Value> = Vec::new();
        values.push(Value::Name("material1".to_string()));
        values.push(Value::ColorAmbient(Color{r: 1.0, g: 2.0, b: 3.0}));
        values.push(Value::ColorDiffuse(Color{r: 2.0, g: 3.0, b: 4.0}));
        values.push(Value::ColorSpecular(Color{r: 3.0, g: 4.0, b: 5.0}));
        values.push(Value::ColorTransmission(Color{r: 4.0, g: 5.0, b: 6.0}));
        values.push(Value::Illum(Illumination::ColorOnAmbientOff));
        values.push(Value::Alpha(1.1));
        values.push(Value::SpecularCoefficient(2.2));
        values.push(Value::OpticalDensity(3.3));
        
        values.push(Value::OpticalDensity(3.4));
        assert_eq!(Err("Duplicate optical density found while constructing Material.\n".to_string()), construct_material_struct(&values));
        values.push(Value::OpticalDensity(3.5));
        assert_eq!(Err("Duplicate optical density found while constructing Material.\n".to_string()), construct_material_struct(&values));
    }

    #[test]
    fn parse_name_value_should_parse_properly() {
        let test_case_1 = &b" newmtl materialname3
"[..];
        let test_case_2 = &b" newmtl materialname3 # Comment
"[..];
        let test_case_3 = &b" newmtl materialname3.001 # Comment
"[..];
        let test_case_4 = &b" newmtl materialname3 after
"[..];
        let test_case_5 = &b" before newmtl materialname3
"[..];

        assert_eq!(Done(&b""[..], Value::Name("materialname3".to_string())), parse_name_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::Name("materialname3".to_string())), parse_name_value(test_case_2));
        assert_eq!(Done(&b""[..], Value::Name("materialname3.001".to_string())), parse_name_value(test_case_3));
        assert_nom_error!(parse_name_value(test_case_4));
        assert_nom_error!(parse_name_value(test_case_5));
    }

    #[test]
    fn parse_color_ambient_value_should_parse_properly() {
        let test_case_1 = &b" Ka 1.1 -2.2 3.3
"[..];
        let test_case_2 = &b" Ka -1.1
"[..];
        let test_case_3 = &b" Ka 4 3 2 # Comment
"[..];
        let test_case_4 = &b" Ka 1.1 2.2 3.3 after
"[..];
        let test_case_5 = &b" before Ka 1.1 2.2 3.3
"[..];

        assert_eq!(Done(&b""[..], Value::ColorAmbient(Color{r: 1.1, g: -2.2, b: 3.3})), parse_color_ambient_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::ColorAmbient(Color{r: -1.1, g: -1.1, b: -1.1})), parse_color_ambient_value(test_case_2));
        assert_eq!(Done(&b""[..], Value::ColorAmbient(Color{r: 4.0, g: 3.0, b: 2.0})), parse_color_ambient_value(test_case_3));
        assert_nom_error!(parse_color_ambient_value(test_case_4));
        assert_nom_error!(parse_color_ambient_value(test_case_5));
    }

    #[test]
    fn parse_color_diffuse_value_should_parse_properly() {
        let test_case_1 = &b" Kd 1.1 2.2 3.3
"[..];
        let test_case_2 = &b" Kd 1.1
"[..];
        let test_case_3 = &b" Kd 4 3 2 # Comment
"[..];
        let test_case_4 = &b" Kd 1.1 2.2 3.3 after
"[..];
        let test_case_5 = &b" before Kd 1.1 2.2 3.3
"[..];

        assert_eq!(Done(&b""[..], Value::ColorDiffuse(Color{r: 1.1, g: 2.2, b: 3.3})), parse_color_diffuse_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::ColorDiffuse(Color{r: 1.1, g: 1.1, b: 1.1})), parse_color_diffuse_value(test_case_2));
        assert_eq!(Done(&b""[..], Value::ColorDiffuse(Color{r: 4.0, g: 3.0, b: 2.0})), parse_color_diffuse_value(test_case_3));
        assert_nom_error!(parse_color_diffuse_value(test_case_4));
        assert_nom_error!(parse_color_diffuse_value(test_case_5));
    }

    #[test]
    fn parse_color_specular_value_should_parse_properly() {
        let test_case_1 = &b" Ks 1.1 2.2 3.3
"[..];
        let test_case_2 = &b" Ks 1.1
"[..];
        let test_case_3 = &b" Ks 4 3 2 # Comment
"[..];
        let test_case_4 = &b" Ks 1.1 2.2 3.3 after
"[..];
        let test_case_5 = &b" before Ks 1.1 2.2 3.3
"[..];

        assert_eq!(Done(&b""[..], Value::ColorSpecular(Color{r: 1.1, g: 2.2, b: 3.3})), parse_color_specular_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::ColorSpecular(Color{r: 1.1, g: 1.1, b: 1.1})), parse_color_specular_value(test_case_2));
        assert_eq!(Done(&b""[..], Value::ColorSpecular(Color{r: 4.0, g: 3.0, b: 2.0})), parse_color_specular_value(test_case_3));
        assert_nom_error!(parse_color_specular_value(test_case_4));
        assert_nom_error!(parse_color_specular_value(test_case_5));
    }

    #[test]
    fn parse_color_transmission_value_should_parse_properly() {
        let test_case_1 = &b" Tf 1.1 2.2 3.3
"[..];
        let test_case_2 = &b" Tf 1.1
"[..];
        let test_case_3 = &b" Tf 4 3 2 # Comment
"[..];
        let test_case_4 = &b" Tf 1.1 2.2 3.3 after
"[..];
        let test_case_5 = &b" before Tf 1.1 2.2 3.3
"[..];

        assert_eq!(Done(&b""[..], Value::ColorTransmission(Color{r: 1.1, g: 2.2, b: 3.3})), parse_color_transmission_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::ColorTransmission(Color{r: 1.1, g: 1.1, b: 1.1})), parse_color_transmission_value(test_case_2));
        assert_eq!(Done(&b""[..], Value::ColorTransmission(Color{r: 4.0, g: 3.0, b: 2.0})), parse_color_transmission_value(test_case_3));
        assert_nom_error!(parse_color_transmission_value(test_case_4));
        assert_nom_error!(parse_color_transmission_value(test_case_5));
    }

    #[test]
    fn parse_illum_value_should_parse_properly() {
        let test_case_1 = &b" illum 0
"[..];
        let test_case_2 = &b" illum 1
"[..];
        let test_case_3 = &b" illum 2
"[..];
        let test_case_4 = &b" illum 3
"[..];
        let test_case_5 = &b" illum 4
"[..];
        let test_case_6 = &b" illum 5
"[..];
        let test_case_7 = &b" illum 6
"[..];
        let test_case_8 = &b" illum 7
"[..];
        let test_case_9 = &b" illum 8
"[..];
        let test_case_10 = &b" illum 9
"[..];
        let test_case_11 = &b" illum 10 # Comment
"[..];
        let test_case_12 = &b" illum 11
"[..];
        let test_case_13 = &b" illum 0 after
"[..];
        let test_case_14 = &b" before illum 0
"[..];

        assert_eq!(Done(&b""[..], Value::Illum(Illumination::ColorOnAmbientOff)), parse_illum_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::ColorOnAmbientOn)), parse_illum_value(test_case_2));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::HighlightOn)), parse_illum_value(test_case_3));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::ReflectionOnAndRayTraceOn)), parse_illum_value(test_case_4));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::TransparencyGlassOnReflectionRayTraceOn)), parse_illum_value(test_case_5));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::ReflectionFresnelOnAndRayTraceOn)), parse_illum_value(test_case_6));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::TransparencyRefractionOnReflectionFresnelOffAndRayTraceOn)), parse_illum_value(test_case_7));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::TransparencyRefractionOnReflectionFresnelOnAndRayTraceOn)), parse_illum_value(test_case_8));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::TeflectionOnAndRayTraceOff)), parse_illum_value(test_case_9));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::TransparencyGlassOnReflectionRayTraceOff)), parse_illum_value(test_case_10));
        assert_eq!(Done(&b""[..], Value::Illum(Illumination::CastsShadowsOntoInvisibleSurfaces)), parse_illum_value(test_case_11));
        assert_nom_error!(parse_color_transmission_value(test_case_12));
        assert_nom_error!(parse_color_transmission_value(test_case_13));
        assert_nom_error!(parse_color_transmission_value(test_case_14));
    }

    #[test]
    fn parse_alpha_value_should_parse_properly() {
        let test_case_1 = &b" d 1.1 # Comment
"[..];
        let test_case_2 = &b" d 1.1 after
"[..];
        let test_case_3 = &b" before d 1.1
"[..];

        assert_eq!(Done(&b""[..], Value::Alpha(1.1)), parse_alpha_value(test_case_1));
        assert_nom_error!(parse_alpha_value(test_case_2));
        assert_nom_error!(parse_alpha_value(test_case_3));
    }

    #[test]
    fn parse_specular_coefficient_should_parse_properly() {
        let test_case_1 = &b" Ns 1.1 # Comment
"[..];
        let test_case_2 = &b" Ns 1.1 after
"[..];
        let test_case_3 = &b" before Ns 1.1
"[..];

        assert_eq!(Done(&b""[..], Value::SpecularCoefficient(1.1)), parse_specular_coefficient_value(test_case_1));
        assert_nom_error!(parse_specular_coefficient_value(test_case_2));
        assert_nom_error!(parse_specular_coefficient_value(test_case_3));
    }

    #[test]
    fn parse_optical_density_should_parse_properly() {
        let test_case_1 = &b" Ni 1.1 # Comment
"[..];
        let test_case_2 = &b" Ni 1.1 after
"[..];
        let test_case_3 = &b" before Ni 1.1
"[..];

        assert_eq!(Done(&b""[..], Value::OpticalDensity(1.1)), parse_optical_density_value(test_case_1));
        assert_nom_error!(parse_optical_density_value(test_case_2));
        assert_nom_error!(parse_optical_density_value(test_case_3));
    }
}
