use nom::IResult::*;

mod material_parser;

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

pub fn parse_materials(string: &str) -> Result<Vec<Material>, String> {
    material_parser::parse_materials(string)
}
