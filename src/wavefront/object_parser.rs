use nom::{space};
use wavefront::parser_utilities::{parse_f64, parse_int, parse_ignored_line, not_space};
use std::str;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex{x: 0.0, y: 0.0, z: 0.0}
    }
}

#[derive(Debug, PartialEq)]
pub struct Triangle {
    v1: VertexTriplet,
    v2: VertexTriplet,
    v3: VertexTriplet,
}

#[derive(Debug, PartialEq)]
pub struct VertexTriplet {
    v: isize,
    vt: Option<isize>,
    vn: Option<isize>,
}

#[derive(Debug, PartialEq)]
enum Value {
    MaterialLibraryName(String),
    UseMaterialName(String),
    VertexGeometric(Vertex),
    VertexNormal(Vertex),
    FaceTriangle(Triangle),
    Group(Vec<String>),
    SmoothingGroup(isize),
}

named!(parse_material_library_value<Value>,
    chain!(
        name: parse_material_library_name,

        ||{Value::MaterialLibraryName(name)}
    )
);

named!(parse_material_library_name<String>,
    chain!(
        many0!(space) ~
        tag!("mtllib") ~
        many0!(space) ~
        name: map_res!(not_space, str::from_utf8) ~
        parse_ignored_line,

        ||{name.to_string()}
    )
);

named!(parse_use_material_value<Value>,
    chain!(
        name: parse_use_material_name,

        ||{Value::UseMaterialName(name)}
    )
);

named!(parse_use_material_name<String>,
    chain!(
        many0!(space) ~
        tag!("usemtl") ~
        many0!(space) ~
        name: map_res!(not_space, str::from_utf8) ~
        parse_ignored_line,

        ||{name.to_string()}
    )
);

named!(parse_vertex_geometry_value<Value>,
    chain!(
        vertex: parse_vertex_geometry,

        ||{Value::VertexGeometric(vertex)}
    )
);

named!(parse_vertex_geometry<Vertex>,
    chain!(
        many0!(space) ~
        tag!("v") ~
        vertex: parse_vertex ~
        parse_ignored_line,

        ||{vertex}
    )
);

named!(parse_vertex_normal_value<Value>,
    chain!(
        vertex: parse_vertex_normal,

        ||{Value::VertexNormal(vertex)}
    )
);

named!(parse_vertex_normal<Vertex>,
    chain!(
        many0!(space) ~
        tag!("vn") ~
        vertex: parse_vertex ~
        parse_ignored_line,

        ||{vertex}
    )
);

named!(parse_vertex<Vertex>,
    chain!(
        many0!(space) ~
        x: parse_f64 ~
        many0!(space) ~
        y: parse_f64 ~
        many0!(space) ~
        z: parse_f64,

        ||{Vertex{x: x, y: y, z: z}}
    )
);

named!(parse_triangle_value<Value>,
    chain!(
        triangle: parse_triangle,

        ||{Value::FaceTriangle(triangle)}
    )
);

named!(parse_triangle<Triangle>,
    chain!(
        many0!(space) ~
        tag!("f") ~
        many0!(space) ~
        v1: parse_vertex_triplet ~
        many0!(space) ~
        v2: parse_vertex_triplet ~
        many0!(space) ~
        v3: parse_vertex_triplet ~
        parse_ignored_line,

        ||{Triangle{v1: v1, v2: v2, v3: v3}}
    )
);

named!(parse_vertex_triplet<VertexTriplet>,
    chain!(
        many0!(space) ~
        v: parse_int ~
        opt!(tag!("/")) ~
        vt : opt!(parse_int) ~
        opt!(tag!("/")) ~
        vn: opt!(parse_int),

        ||{
            let mut vertex_texture: Option<isize> = None;
            let mut vertex_normal: Option<isize> = None;
            if let Some(vt) = vt {
                vertex_texture = Some(vt)
            }
            if let Some(vn) = vn {
                vertex_normal = Some(vn)
            }
            VertexTriplet{v: v, vt: vertex_texture, vn: vertex_normal}}
    )
);

named!(parse_group_value<Value>,
    chain!(
        names: parse_group_names,

        ||{Value::Group(names)}
    )
);

named!(parse_group_names<Vec<String> >,
    chain!(
        many0!(space) ~
        tag!("g") ~
        many0!(space) ~
        names: many1!(
            chain!(
                name: map_res!(not_space, str::from_utf8) ~
                many0!(space),

                ||{name.to_string()}
            )
        ) ~
        parse_ignored_line,

        ||{names}
    )
);

named!(parse_smoothing_group_value<Value>,
    chain!(
        value: parse_smoothing_group,

        ||{Value::SmoothingGroup(value)}
    )
);

named!(parse_smoothing_group<isize>,
    chain!(
        many0!(space) ~
        tag!("s") ~
        many0!(space) ~
        value: parse_int ~
        parse_ignored_line,

        ||{value}
    )
);

#[cfg(test)]
mod tests
{
    use nom::IResult;
    use nom::IResult::*;
    use super::{Vertex,
                Triangle,
                VertexTriplet,
                Value,
                parse_material_library_value,
                parse_use_material_value,
                parse_vertex_geometry_value,
                parse_vertex_normal_value,
                parse_triangle_value,
                parse_group_value,
                parse_smoothing_group_value,};

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
    fn parse_material_library_value_should_parse_properly() {
        let test_case_1 = &b" mtllib Material.Library
"[..];
        let test_case_2 = &b" mtllib Material.Library after
"[..];
        let test_case_3 = &b" before mtllib Material.Library
"[..];

        assert_eq!(Done(&b""[..], Value::MaterialLibraryName("Material.Library".to_string())), parse_material_library_value(test_case_1));
        assert_nom_error!(parse_material_library_value(test_case_2));
        assert_nom_error!(parse_material_library_value(test_case_3));
    }

    #[test]
    fn parse_use_material_value_should_parse_properly() {
        let test_case_1 = &b" usemtl Material.01
"[..];
        let test_case_2 = &b" mtllib Material.01 after
"[..];
        let test_case_3 = &b" before mtllib Material.01
"[..];

        assert_eq!(Done(&b""[..], Value::UseMaterialName("Material.01".to_string())), parse_use_material_value(test_case_1));
        assert_nom_error!(parse_use_material_value(test_case_2));
        assert_nom_error!(parse_use_material_value(test_case_3));
    }

    #[test]
    fn parse_vertex_geometry_value_should_parse_properly() {
        let test_case_1 = &b" v 1.0 2.0 -3.0
"[..];
        let test_case_2 = &b" v 1.0 2.0 3.0 after
"[..];
        let test_case_3 = &b" before v 1.0 2.0 3.0
"[..];

        assert_eq!(Done(&b""[..], Value::VertexGeometric(Vertex{x: 1.0, y: 2.0, z: -3.0})), parse_vertex_geometry_value(test_case_1));
        assert_nom_error!(parse_vertex_geometry_value(test_case_2));
        assert_nom_error!(parse_vertex_geometry_value(test_case_3));
    }

    #[test]
    fn parse_vertex_normal_value_should_parse_properly() {
        let test_case_1 = &b" vn 1.0 2.0 -3.0
"[..];
        let test_case_2 = &b" vn 1.0 2.0 3.0 after
"[..];
        let test_case_3 = &b" before vn 1.0 2.0 3.0
"[..];

        assert_eq!(Done(&b""[..], Value::VertexNormal(Vertex{x: 1.0, y: 2.0, z: -3.0})), parse_vertex_normal_value(test_case_1));
        assert_nom_error!(parse_vertex_normal_value(test_case_2));
        assert_nom_error!(parse_vertex_normal_value(test_case_3));
    }

    #[test]
    fn parse_triangle_value_should_parse_properly() {
        parse_triangle_value_should_parse_full_triples();
        parse_triangle_value_should_parse_triples_without_vertex_normals();
        parse_triangle_value_should_parse_triples_without_vertex_textures();
        parse_triangle_value_should_parse_triples_without_vertex_textures_or_vertex_normals();
        parse_triangle_value_should_not_parse_if_improperly_formatted();
    }

    fn parse_triangle_value_should_parse_full_triples() {
        let test_case = &b" f 1/1/1 2/2/2 3/3/3
"[..];

        let test_case_expected = Value::FaceTriangle(
            Triangle{
                v1: VertexTriplet{v: 1, vt: Some(1), vn: Some(1)},
                v2: VertexTriplet{v: 2, vt: Some(2), vn: Some(2)},
                v3: VertexTriplet{v: 3, vt: Some(3), vn: Some(3)},
            }
        );
        assert_eq!(Done(&b""[..], test_case_expected), parse_triangle_value(test_case));
    }

    fn parse_triangle_value_should_parse_triples_without_vertex_normals() {
        let test_case = &b" f 1/1 2/2/ 3/3
"[..];

        let test_case_expected = Value::FaceTriangle(
            Triangle{
                v1: VertexTriplet{v: 1, vt: Some(1), vn: None},
                v2: VertexTriplet{v: 2, vt: Some(2), vn: None},
                v3: VertexTriplet{v: 3, vt: Some(3), vn: None},
            }
        );
        assert_eq!(Done(&b""[..], test_case_expected), parse_triangle_value(test_case));
    }

    fn parse_triangle_value_should_parse_triples_without_vertex_textures() {
        let test_case = &b" f 1//1 2//2 3//3
"[..];

        let test_case_expected = Value::FaceTriangle(
            Triangle{
                v1: VertexTriplet{v: 1, vt: None, vn: Some(1)},
                v2: VertexTriplet{v: 2, vt: None, vn: Some(2)},
                v3: VertexTriplet{v: 3, vt: None, vn: Some(3)},
            }
        );
        assert_eq!(Done(&b""[..], test_case_expected), parse_triangle_value(test_case));
    }

    fn parse_triangle_value_should_parse_triples_without_vertex_textures_or_vertex_normals() {
        let test_case = &b" f 1 2/ 3//
"[..];

        let test_case_expected = Value::FaceTriangle(
            Triangle{
                v1: VertexTriplet{v: 1, vt: None, vn: None},
                v2: VertexTriplet{v: 2, vt: None, vn: None},
                v3: VertexTriplet{v: 3, vt: None, vn: None},
            }
        );
        assert_eq!(Done(&b""[..], test_case_expected), parse_triangle_value(test_case));
    }

    fn parse_triangle_value_should_not_parse_if_improperly_formatted() {
        let test_case_1 = &b" f 1 2
"[..];
        let test_case_2 = &b" f 1
"[..];
        let test_case_3 = &b" f 1 2 3 4
"[..];
        let test_case_4 = &b" f 1 2 3/3/3/
"[..];
        assert_nom_error!(parse_triangle_value(test_case_1));
        assert_nom_error!(parse_triangle_value(test_case_2));
        assert_nom_error!(parse_triangle_value(test_case_3));
        assert_nom_error!(parse_triangle_value(test_case_4));
    }

    #[test]
    fn parse_group_value_should_parse_properly() {
        let test_case_1 = &b" g Group.1 Group.2 MaterialGroup.3.Yes!
"[..];
        let test_case_2 = &b" g Group.1
"[..];
        let test_case_3 = &b" before g Group1
"[..];

        assert_eq!(Done(&b""[..], Value::Group(vec!["Group.1".to_string(), "Group.2".to_string(), "MaterialGroup.3.Yes!".to_string()])), parse_group_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::Group(vec!["Group.1".to_string()])), parse_group_value(test_case_2));
        assert_nom_error!(parse_vertex_normal_value(test_case_3));
    }

    #[test]
    fn parse_smoothing_group_value_should_parse_properly() {
        let test_case_1 = &b" s 0
"[..];
        let test_case_2 = &b" s 1
"[..];
        let test_case_3 = &b" before s 0
"[..];
        let test_case_4 = &b" s 0 1
"[..];

        assert_eq!(Done(&b""[..], Value::SmoothingGroup(0)), parse_smoothing_group_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::SmoothingGroup(1)), parse_smoothing_group_value(test_case_2));
        assert_nom_error!(parse_vertex_normal_value(test_case_3));
        assert_nom_error!(parse_vertex_normal_value(test_case_4));
    }
}
