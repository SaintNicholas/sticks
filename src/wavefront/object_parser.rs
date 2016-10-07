use nom::{space};
use nom::IResult::*;
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
    v1: Vertex,
    v2: Vertex,
    v3: Vertex,
    vt1: Option<Vertex>,
    vt2: Option<Vertex>,
    vt3: Option<Vertex>,
    vn1: Option<Vertex>,
    vn2: Option<Vertex>,
    vn3: Option<Vertex>,
    materialName: String,
}

impl Default for Triangle {
    fn default() -> Triangle {
        Triangle{
            v1: Default::default(),
            v2: Default::default(), 
            v3: Default::default(),
            vt1: None,
            vt2: None,
            vt3: None,
            vn1: None,
            vn2: None,
            vn3: None,
            materialName: "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TriangleOfVertexTriplets {
    v1: VertexTriplet,
    v2: VertexTriplet,
    v3: VertexTriplet,
}

impl Default for TriangleOfVertexTriplets {
    fn default() -> TriangleOfVertexTriplets {
        TriangleOfVertexTriplets{
            v1: VertexTriplet{
                v: 0,
                vt: None,
                vn: None
            },
            v2: VertexTriplet{
                v: 0,
                vt: None,
                vn: None
            },
            v3: VertexTriplet{
                v: 0,
                vt: None,
                vn: None
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VertexTriplet {
    v: isize,
    vt: Option<isize>,
    vn: Option<isize>,
}

#[derive(Debug, PartialEq)]
enum Value {
    ValueMaterialLibraryName(String),
    ValueUseMaterialName(String),
    ValueVertexGeometric(Vertex),
    ValueVertexTexture(Vertex),
    ValueVertexNormal(Vertex),
    ValueTriangle(TriangleOfVertexTriplets),
    ValueGroup(Vec<String>),
    ValueSmoothingValueGroup(isize),
}

#[derive(Debug, PartialEq)]
pub struct Object {
    triangles: Vec<Triangle>,
    raw_vertices: Vec<Vertex>,
    raw_vertices_texture: Vec<Vertex>,
    raw_vertices_normals: Vec<Vertex>,
}

impl Default for Object {
    fn default() -> Object {
        Object{
            triangles: vec![],
            raw_vertices: vec![],
            raw_vertices_texture: vec![],
            raw_vertices_normals: vec![],
        }
    }
}

pub fn parse_object(string: &str) -> Result<Object, String> {
    let result = parse_values(string.as_bytes());
    if let Done(remaining, parsed) = result {
        if remaining == [] {
            construct_object_struct(&parsed)
        } else {
            Err(format!("Parser error: Failed parsing everything. Leftover: {:?}", remaining))
        }
    } else if let Error(error) = result {
        Err(format!("Parser error: Error {:?}", error))
    } else if let Incomplete(remaining) = result {
        Err(format!("Parser error: Parsing incomplete."))
    } else {
        unimplemented!()
    }
}

fn construct_object_struct(values: &Vec<Value>) -> Result<Object, String> {
    let mut object: Object = Default::default();
    let mut materialName: String = "".to_string();

    'parsing_values: for value in values {
        match value {
            &Value::ValueUseMaterialName(ref value_name) => {
                materialName = value_name.clone();
            }
            &Value::ValueVertexGeometric(ref value_vertex) => {
                object.raw_vertices.push(*value_vertex);
            }
            &Value::ValueVertexTexture(ref value_vertex) => {
                object.raw_vertices_texture.push(*value_vertex);
            }
            &Value::ValueVertexNormal(ref value_vertex) => {
                object.raw_vertices_normals.push(*value_vertex);
            }
            &Value::ValueTriangle(ref triangle_of_triplets) => {
                let mut triangle: Triangle = try!(construct_triangle_from_triangle_of_triplets(&object, triangle_of_triplets, &materialName));
                object.triangles.push(triangle);
            }
            _ => {
                // Nothing
            }
        }
    }

    Ok(object)
}

fn construct_triangle_from_triangle_of_triplets(object: &Object, triangle_of_triplets: &TriangleOfVertexTriplets, material_name: &String) -> Result<Triangle, String> {
    let mut triangle: Triangle = Default::default();

    triangle.v1 = try!(get_indexed_vertex(&object.raw_vertices, triangle_of_triplets.v1.v));
    triangle.v2 = try!(get_indexed_vertex(&object.raw_vertices, triangle_of_triplets.v2.v));
    triangle.v3 = try!(get_indexed_vertex(&object.raw_vertices, triangle_of_triplets.v3.v));

    if let Some(v) = triangle_of_triplets.v1.vt {
        triangle.vt1 = Some(try!(get_indexed_vertex(&object.raw_vertices_texture, v)));
    }
    if let Some(v) = triangle_of_triplets.v2.vt {
        triangle.vt2 = Some(try!(get_indexed_vertex(&object.raw_vertices_texture, v)));
    }
    if let Some(v) = triangle_of_triplets.v3.vt {
        triangle.vt3 = Some(try!(get_indexed_vertex(&object.raw_vertices_texture, v)));
    }

    if let Some(v) = triangle_of_triplets.v1.vn {
        triangle.vn1 = Some(try!(get_indexed_vertex(&object.raw_vertices_normals, v)));
    }
    if let Some(v) = triangle_of_triplets.v2.vn {
        triangle.vn2 = Some(try!(get_indexed_vertex(&object.raw_vertices_normals, v)));
    }
    if let Some(v) = triangle_of_triplets.v3.vn {
        triangle.vn3 = Some(try!(get_indexed_vertex(&object.raw_vertices_normals, v)));
    }

    triangle.materialName = material_name.clone();

    Ok(triangle)
}

fn get_indexed_vertex(list: &Vec<Vertex>, index: isize) -> Result<Vertex, String> {
    let num_vertices = list.len() as isize;
    let mut usize_index = 0;

    if index > 0 && index <= num_vertices {
        usize_index = (index - 1) as usize;
    } else if index < 0 && index >= -num_vertices {
        usize_index = (num_vertices + index) as usize;
    } else {
        return Err("Invalid index".to_string());
    }

    Ok(list[usize_index])
}

named!(parse_values<Vec<Value> >,
    many0!(
        chain!(
            many0!(parse_ignored_line) ~
            value: alt!(
                parse_material_library_name_value |
                parse_use_material_value |
                parse_vertex_geometry_value |
                parse_vertex_normal_value |
                parse_triangle_value |
                parse_ValueGroup_value |
                parse_smoothing_ValueGroup_value
            ) ~
            many0!(parse_ignored_line),

            ||{value}
        )
    )
);

named!(parse_material_library_name_value<Value>,
    chain!(
        name: parse_material_library_name,

        ||{Value::ValueMaterialLibraryName(name)}
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

        ||{Value::ValueUseMaterialName(name)}
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

        ||{Value::ValueVertexGeometric(vertex)}
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

        ||{Value::ValueVertexNormal(vertex)}
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

        ||{Value::ValueTriangle(triangle)}
    )
);

named!(parse_triangle<TriangleOfVertexTriplets>,
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

        ||{TriangleOfVertexTriplets{v1: v1, v2: v2, v3: v3}}
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

named!(parse_ValueGroup_value<Value>,
    chain!(
        names: parse_ValueGroup_names,

        ||{Value::ValueGroup(names)}
    )
);

named!(parse_ValueGroup_names<Vec<String> >,
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

named!(parse_smoothing_ValueGroup_value<Value>,
    chain!(
        value: parse_smoothing_ValueGroup,

        ||{Value::ValueSmoothingValueGroup(value)}
    )
);

named!(parse_smoothing_ValueGroup<isize>,
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
                TriangleOfVertexTriplets,
                VertexTriplet,
                Value,
                Object,
                parse_object,
                construct_object_struct,
                construct_triangle_from_triangle_of_triplets,
                get_indexed_vertex,
                parse_values,
                parse_material_library_name_value,
                parse_use_material_value,
                parse_vertex_geometry_value,
                parse_vertex_normal_value,
                parse_triangle_value,
                parse_ValueGroup_value,
                parse_smoothing_ValueGroup_value,};

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
    fn parse_object_should_do_things_correctly() {
        test_parse_object_should_parse_object_file();
        test_parse_object_should_return_error_if_parsed_properly_but_constructing_fails();
        test_parse_object_should_return_error_if_parsing_returned_done_but_leftover_data();
        test_parse_object_should_return_error_if_parsing_returned_error();
        test_parse_object_should_return_error_if_parsing_returned_incomplete();
    }

    fn test_parse_object_should_parse_object_file() {
        let test_case = "v 1.0 2.0 -3.0
f 1// 1// 1//
";
        let expected_object: Object = Object{
            triangles: vec![
                Triangle{
                    v1: Vertex{x: 1.0, y: 2.0, z: -3.0},
                    v2: Vertex{x: 1.0, y: 2.0, z: -3.0},
                    v3: Vertex{x: 1.0, y: 2.0, z: -3.0},
                    vt1: None,
                    vt2: None,
                    vt3: None,
                    vn1: None,
                    vn2: None,
                    vn3: None,
                    materialName: "".to_string()
                }
            ],
            raw_vertices: vec![
                Vertex{x: 1.0, y: 2.0, z: -3.0},
            ],
            raw_vertices_texture: vec![],
            raw_vertices_normals: vec![],
        };

        assert_eq!(Ok(expected_object), parse_object(test_case));
    }

    fn test_parse_object_should_return_error_if_parsed_properly_but_constructing_fails() {
        let test_case = "v 1.0 2.0 -3.0
f 0// 1// 1//
";

        assert_eq!(Err("Invalid index".to_string()), parse_object(test_case));
    }

    fn test_parse_object_should_return_error_if_parsing_returned_done_but_leftover_data() {
        let test_case = "v 1.0 2.0 -3.0
f 1// 1// 1//
l
";

        let remaining = ['l' as u8, '\n' as u8];
        assert_eq!(Err(format!("Parser error: Failed parsing everything. Leftover: {:?}", remaining)), parse_object(test_case));
    }

    fn test_parse_object_should_return_error_if_parsing_returned_error() {
    }

    fn test_parse_object_should_return_error_if_parsing_returned_incomplete() {
    }

    #[test]
    fn construct_object_struct_should_do_things_correctly() {
        let values: Vec<Value> = vec![
            Value::ValueMaterialLibraryName("Material.Library".to_string()),
            Value::ValueUseMaterialName("Material.01".to_string()),
            Value::ValueVertexGeometric(Vertex{x: -1.0, y: 1.0, z: 1.0}),
            Value::ValueVertexGeometric(Vertex{x: -2.0, y: 2.0, z: 2.0}),
            Value::ValueVertexGeometric(Vertex{x: -3.0, y: 3.0, z: 3.0}),
            Value::ValueVertexGeometric(Vertex{x: -4.0, y: 4.0, z: 4.0}),
            Value::ValueVertexGeometric(Vertex{x: -5.0, y: 5.0, z: 5.0}),
            Value::ValueVertexGeometric(Vertex{x: -6.0, y: 6.0, z: 6.0}),
            Value::ValueVertexGeometric(Vertex{x: -7.0, y: 7.0, z: 7.0}),
            Value::ValueVertexGeometric(Vertex{x: -8.0, y: 8.0, z: 8.0}),
            Value::ValueVertexGeometric(Vertex{x: -9.0, y: 9.0, z: 9.0}),
            Value::ValueVertexGeometric(Vertex{x: -10.0, y: 10.0, z: 10.0}),
            Value::ValueVertexTexture(Vertex{x: 1.1, y: -1.1, z: 1.1}),
            Value::ValueVertexTexture(Vertex{x: 2.1, y: -2.1, z: 2.1}),
            Value::ValueVertexTexture(Vertex{x: 3.1, y: -3.1, z: 3.1}),
            Value::ValueVertexTexture(Vertex{x: 4.1, y: -4.1, z: 4.1}),
            Value::ValueVertexTexture(Vertex{x: 5.1, y: -5.1, z: 5.1}),
            Value::ValueVertexTexture(Vertex{x: 6.1, y: -6.1, z: 6.1}),
            Value::ValueVertexTexture(Vertex{x: 7.1, y: -7.1, z: 7.1}),
            Value::ValueVertexTexture(Vertex{x: 8.1, y: -8.1, z: 8.1}),
            Value::ValueVertexTexture(Vertex{x: 9.1, y: -9.1, z: 9.1}),
            Value::ValueVertexTexture(Vertex{x: 10.1, y: -10.1, z: 10.1}),
            Value::ValueVertexNormal(Vertex{x: 1.2, y: 1.2, z: -1.2}),
            Value::ValueVertexNormal(Vertex{x: 2.2, y: 2.2, z: -2.2}),
            Value::ValueVertexNormal(Vertex{x: 3.2, y: 3.2, z: -3.2}),
            Value::ValueVertexNormal(Vertex{x: 4.2, y: 4.2, z: -4.2}),
            Value::ValueVertexNormal(Vertex{x: 5.2, y: 5.2, z: -5.2}),
            Value::ValueVertexNormal(Vertex{x: 6.2, y: 6.2, z: -6.2}),
            Value::ValueVertexNormal(Vertex{x: 7.2, y: 7.2, z: -7.2}),
            Value::ValueVertexNormal(Vertex{x: 8.2, y: 8.2, z: -8.2}),
            Value::ValueVertexNormal(Vertex{x: 9.2, y: 9.2, z: -9.2}),
            Value::ValueVertexNormal(Vertex{x: 10.2, y: 10.2, z: -10.2}),
            Value::ValueSmoothingValueGroup(0),
            Value::ValueGroup(vec!["ValueGroup.2".to_string()]),
            Value::ValueTriangle(TriangleOfVertexTriplets{
                v1: VertexTriplet{
                    v: -1,
                    vt: Some(3),
                    vn: Some(10)
                },
                v2: VertexTriplet{
                    v: 1,
                    vt: Some(-4),
                    vn: Some(6)
                },
                v3: VertexTriplet{
                    v: 3,
                    vt: Some(5),
                    vn: Some(-7)
                }
            }),
            Value::ValueVertexGeometric(Vertex{x: -11.0, y: 11.0, z: 11.0}),
            Value::ValueVertexTexture(Vertex{x: 11.1, y: -11.1, z: 11.1}),
            Value::ValueVertexNormal(Vertex{x: 11.2, y: 11.2, z: -11.2}),
            Value::ValueUseMaterialName("Material.02".to_string()),
            Value::ValueTriangle(TriangleOfVertexTriplets{
                v1: VertexTriplet{
                    v: 1,
                    vt: None,
                    vn: None
                },
                v2: VertexTriplet{
                    v: -11,
                    vt: None,
                    vn: None
                },
                v3: VertexTriplet{
                    v: -1,
                    vt: None,
                    vn: None
                }
            })
        ];

        let expected_object: Object = Object{
            triangles: vec![
                Triangle{
                    v1: Vertex{x: -10.0, y: 10.0, z: 10.0},
                    v2: Vertex{x: -1.0, y: 1.0, z: 1.0},
                    v3: Vertex{x: -3.0, y: 3.0, z: 3.0},
                    vt1: Some(Vertex{x: 3.1, y: -3.1, z: 3.1}),
                    vt2: Some(Vertex{x: 7.1, y: -7.1, z: 7.1}),
                    vt3: Some(Vertex{x: 5.1, y: -5.1, z: 5.1}),
                    vn1: Some(Vertex{x: 10.2, y: 10.2, z: -10.2}),
                    vn2: Some(Vertex{x: 6.2, y: 6.2, z: -6.2}),
                    vn3: Some(Vertex{x: 4.2, y: 4.2, z: -4.2}),
                    materialName: "Material.01".to_string()
                },
                Triangle{
                    v1: Vertex{x: -1.0, y: 1.0, z: 1.0},
                    v2: Vertex{x: -1.0, y: 1.0, z: 1.0},
                    v3: Vertex{x: -11.0, y: 11.0, z: 11.0},
                    vt1: None,
                    vt2: None,
                    vt3: None,
                    vn1: None,
                    vn2: None,
                    vn3: None,
                    materialName: "Material.02".to_string()
                }
            ],
            raw_vertices: vec![
                Vertex{x: -1.0, y: 1.0, z: 1.0},
                Vertex{x: -2.0, y: 2.0, z: 2.0},
                Vertex{x: -3.0, y: 3.0, z: 3.0},
                Vertex{x: -4.0, y: 4.0, z: 4.0},
                Vertex{x: -5.0, y: 5.0, z: 5.0},
                Vertex{x: -6.0, y: 6.0, z: 6.0},
                Vertex{x: -7.0, y: 7.0, z: 7.0},
                Vertex{x: -8.0, y: 8.0, z: 8.0},
                Vertex{x: -9.0, y: 9.0, z: 9.0},
                Vertex{x: -10.0, y: 10.0, z: 10.0},
                Vertex{x: -11.0, y: 11.0, z: 11.0}
            ],
            raw_vertices_texture: vec![
                Vertex{x: 1.1, y: -1.1, z: 1.1},
                Vertex{x: 2.1, y: -2.1, z: 2.1},
                Vertex{x: 3.1, y: -3.1, z: 3.1},
                Vertex{x: 4.1, y: -4.1, z: 4.1},
                Vertex{x: 5.1, y: -5.1, z: 5.1},
                Vertex{x: 6.1, y: -6.1, z: 6.1},
                Vertex{x: 7.1, y: -7.1, z: 7.1},
                Vertex{x: 8.1, y: -8.1, z: 8.1},
                Vertex{x: 9.1, y: -9.1, z: 9.1},
                Vertex{x: 10.1, y: -10.1, z: 10.1},
                Vertex{x: 11.1, y: -11.1, z: 11.1}
            ],
            raw_vertices_normals: vec![
                Vertex{x: 1.2, y: 1.2, z: -1.2},
                Vertex{x: 2.2, y: 2.2, z: -2.2},
                Vertex{x: 3.2, y: 3.2, z: -3.2},
                Vertex{x: 4.2, y: 4.2, z: -4.2},
                Vertex{x: 5.2, y: 5.2, z: -5.2},
                Vertex{x: 6.2, y: 6.2, z: -6.2},
                Vertex{x: 7.2, y: 7.2, z: -7.2},
                Vertex{x: 8.2, y: 8.2, z: -8.2},
                Vertex{x: 9.2, y: 9.2, z: -9.2},
                Vertex{x: 10.2, y: 10.2, z: -10.2},
                Vertex{x: 11.2, y: 11.2, z: -11.2}
            ],
        };

        assert_eq!(Ok(expected_object), construct_object_struct(&values));
    }

    #[test]
    fn construct_triangle_from_triangle_of_triplets_should_do_things_correctly() {
        let mut triangle_of_triplets: TriangleOfVertexTriplets = Default::default();
        let mut object: Object = Default::default();
        object.raw_vertices.push(Vertex{x: 1.0, y: 2.0, z: 3.0});
        object.raw_vertices.push(Vertex{x: 2.0, y: 3.0, z: 4.0});
        object.raw_vertices.push(Vertex{x: 3.0, y: 4.0, z: 5.0});
        object.raw_vertices.push(Vertex{x: 4.0, y: 5.0, z: 6.0});
        object.raw_vertices.push(Vertex{x: 5.0, y: 6.0, z: 7.0});
        object.raw_vertices_texture.push(Vertex{x: 1.1, y: 2.1, z: 3.1});
        object.raw_vertices_texture.push(Vertex{x: 2.1, y: 3.1, z: 4.1});
        object.raw_vertices_texture.push(Vertex{x: 3.1, y: 4.1, z: 5.1});
        object.raw_vertices_texture.push(Vertex{x: 4.1, y: 5.1, z: 6.1});
        object.raw_vertices_texture.push(Vertex{x: 5.1, y: 6.1, z: 7.1});
        object.raw_vertices_normals.push(Vertex{x: 1.2, y: 2.2, z: 3.2});
        object.raw_vertices_normals.push(Vertex{x: 2.2, y: 3.2, z: 4.2});
        object.raw_vertices_normals.push(Vertex{x: 3.2, y: 4.2, z: 5.2});
        object.raw_vertices_normals.push(Vertex{x: 4.2, y: 5.2, z: 6.2});
        object.raw_vertices_normals.push(Vertex{x: 5.2, y: 6.2, z: 7.2});

        let mut expected_1: Triangle = Default::default();
        expected_1.v1 = Vertex{x: 1.0, y: 2.0, z: 3.0};
        expected_1.v2 = Vertex{x: 2.0, y: 3.0, z: 4.0};
        expected_1.v3 = Vertex{x: 3.0, y: 4.0, z: 5.0};
        expected_1.vt1 = Some(Vertex{x: 2.1, y: 3.1, z: 4.1});
        expected_1.vt2 = Some(Vertex{x: 3.1, y: 4.1, z: 5.1});
        expected_1.vt3 = Some(Vertex{x: 4.1, y: 5.1, z: 6.1});
        expected_1.vn1 = Some(Vertex{x: 3.2, y: 4.2, z: 5.2});
        expected_1.vn2 = Some(Vertex{x: 4.2, y: 5.2, z: 6.2});
        expected_1.vn3 = Some(Vertex{x: 5.2, y: 6.2, z: 7.2});
        expected_1.materialName = "Material".to_string();
        triangle_of_triplets.v1.v = 1;
        triangle_of_triplets.v2.v = 2;
        triangle_of_triplets.v3.v = 3;
        triangle_of_triplets.v1.vt = Some(2);
        triangle_of_triplets.v2.vt = Some(3);
        triangle_of_triplets.v3.vt = Some(4);
        triangle_of_triplets.v1.vn = Some(3);
        triangle_of_triplets.v2.vn = Some(4);
        triangle_of_triplets.v3.vn = Some(5);
        assert_eq!(Ok(expected_1), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        let mut expected_2: Triangle = Default::default();
        expected_2.v1 = Vertex{x: 1.0, y: 2.0, z: 3.0};
        expected_2.v2 = Vertex{x: 2.0, y: 3.0, z: 4.0};
        expected_2.v3 = Vertex{x: 3.0, y: 4.0, z: 5.0};
        expected_2.vt1 = None;
        expected_2.vt2 = None;
        expected_2.vt3 = None;
        expected_2.vn1 = None;
        expected_2.vn2 = None;
        expected_2.vn3 = None;
        expected_2.materialName = "Material".to_string();
        triangle_of_triplets.v1.vt = None;
        triangle_of_triplets.v2.vt = None;
        triangle_of_triplets.v3.vt = None;
        triangle_of_triplets.v1.vn = None;
        triangle_of_triplets.v2.vn = None;
        triangle_of_triplets.v3.vn = None;
        assert_eq!(Ok(expected_2), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v1.v = 0;
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v1.v = 1;
        triangle_of_triplets.v2.v = 0;
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v2.v = 1;
        triangle_of_triplets.v3.v = 0;
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v3.v = 1;
        triangle_of_triplets.v1.vt = Some(0);
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v1.vt = None;
        triangle_of_triplets.v2.vt = Some(0);
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v2.vt = None;
        triangle_of_triplets.v3.vt = Some(0);
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v3.vt = None;
        triangle_of_triplets.v1.vn = Some(0);
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v1.vn = None;
        triangle_of_triplets.v2.vn = Some(0);
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v2.vt = None;
        triangle_of_triplets.v3.vt = Some(0);
        assert_eq!(Err("Invalid index".to_string()), construct_triangle_from_triangle_of_triplets(&object, &triangle_of_triplets, &"Material".to_string()));

        triangle_of_triplets.v3.vt = None;
    }

    #[test]
    fn get_indexed_vertex_should_do_things_correctly() {
        let mut vertices: Vec<Vertex> = Default::default();
        vertices.push(Vertex{x: 1.0, y: 2.0, z: 3.0});
        vertices.push(Vertex{x: 2.0, y: 3.0, z: 4.0});
        vertices.push(Vertex{x: 3.0, y: 4.0, z: 5.0});
        vertices.push(Vertex{x: 4.0, y: 5.0, z: 6.0});
        vertices.push(Vertex{x: 5.0, y: 6.0, z: 7.0});

        assert_eq!(Ok(Vertex{x: 1.0, y: 2.0, z: 3.0}), get_indexed_vertex(&vertices, 1));
        assert_eq!(Ok(Vertex{x: 1.0, y: 2.0, z: 3.0}), get_indexed_vertex(&vertices, -5));
        assert_eq!(Ok(Vertex{x: 5.0, y: 6.0, z: 7.0}), get_indexed_vertex(&vertices, 5));
        assert_eq!(Ok(Vertex{x: 5.0, y: 6.0, z: 7.0}), get_indexed_vertex(&vertices, -1));
        assert_eq!(Err("Invalid index".to_string()), get_indexed_vertex(&vertices, 0));
        assert_eq!(Err("Invalid index".to_string()), get_indexed_vertex(&vertices, 6));
        assert_eq!(Err("Invalid index".to_string()), get_indexed_vertex(&vertices, -6));
    }

    #[test]
    fn parse_values_should_parse_properly() {
        parse_values_should_parse_object_file_properly();
        parse_values_should_return_partially_parsed_result_if_parsing_fails();
    }

    fn parse_values_should_parse_object_file_properly() {
        let test_case = &b"mtllib Material.Library
usemtl Material.01
v 1.0 2.0 -3.0
mtllib Material.Library2
v 1.0 3.0 -3.0
s 0
vn 1.0 2.0 -3.0
g ValueGroup.2
f -1/5/9 2/-6/10 3/7/-11
usemtl Material.02
f 1/1 2/2/ 3/3
v 1.0 2.0 -3.0
f 1//1 2//2 3//3
vn 1.0 2.0 -3.0
f 1 2/ 3//
g ValueGroup.1 ValueGroup.2 MaterialValueGroup.3.Yes!
s 0
"[..];

        let expected: Vec<Value> = vec![
            Value::ValueMaterialLibraryName("Material.Library".to_string()),
            Value::ValueUseMaterialName("Material.01".to_string()),
            Value::ValueVertexGeometric(Vertex{x: 1.0, y: 2.0, z: -3.0}),
            Value::ValueMaterialLibraryName("Material.Library2".to_string()),
            Value::ValueVertexGeometric(Vertex{x: 1.0, y: 3.0, z: -3.0}),
            Value::ValueSmoothingValueGroup(0),
            Value::ValueVertexNormal(Vertex{x: 1.0, y: 2.0, z: -3.0}),
            Value::ValueGroup(vec!["ValueGroup.2".to_string()]),
            Value::ValueTriangle(TriangleOfVertexTriplets{
                v1: VertexTriplet{
                    v: -1,
                    vt: Some(5),
                    vn: Some(9)
                },
                v2: VertexTriplet{
                    v: 2,
                    vt: Some(-6),
                    vn: Some(10)
                },
                v3: VertexTriplet{
                    v: 3,
                    vt: Some(7),
                    vn: Some(-11)
                }
            }),
            Value::ValueUseMaterialName("Material.02".to_string()),
            Value::ValueTriangle(TriangleOfVertexTriplets{
                v1: VertexTriplet{
                    v: 1,
                    vt: Some(1),
                    vn: None
                },
                v2: VertexTriplet{
                    v: 2,
                    vt: Some(2),
                    vn: None
                },
                v3: VertexTriplet{
                    v: 3,
                    vt: Some(3),
                    vn: None
                }
            }),
            Value::ValueVertexGeometric(Vertex{x: 1.0, y: 2.0, z: -3.0}),
            Value::ValueTriangle(TriangleOfVertexTriplets{
                v1: VertexTriplet{
                    v: 1,
                    vt: None,
                    vn: Some(1)
                },
                v2: VertexTriplet{
                    v: 2,
                    vt: None,
                    vn: Some(2)
                },
                v3: VertexTriplet{
                    v: 3,
                    vt: None,
                    vn: Some(3)
                }
            }),
            Value::ValueVertexNormal(Vertex{x: 1.0, y: 2.0, z: -3.0}),
            Value::ValueTriangle(TriangleOfVertexTriplets{
                v1: VertexTriplet{
                    v: 1,
                    vt: None,
                    vn: None
                },
                v2: VertexTriplet{
                    v: 2,
                    vt: None,
                    vn: None
                },
                v3: VertexTriplet{
                    v: 3,
                    vt: None,
                    vn: None
                }
            }),
            Value::ValueGroup(vec!["ValueGroup.1".to_string(), "ValueGroup.2".to_string(), "MaterialValueGroup.3.Yes!".to_string()]),
            Value::ValueSmoothingValueGroup(0),
        ];

        assert_eq!(Done(&b""[..], expected), parse_values(test_case));
    }

    fn parse_values_should_return_partially_parsed_result_if_parsing_fails() {
        let test_case = &b"mtllib Material.Library
usemtl Material.01
l
"[..];
        assert_eq!(Done(&b"l
"[..], vec![Value::ValueMaterialLibraryName("Material.Library".to_string()), Value::ValueUseMaterialName("Material.01".to_string())]), parse_values(test_case));
    }

    #[test]
    fn parse_material_library_name_value_should_parse_properly() {
        let test_case_1 = &b" mtllib Material.Library
"[..];
        let test_case_2 = &b" mtllib Material.Library after
"[..];
        let test_case_3 = &b" before mtllib Material.Library
"[..];

        assert_eq!(Done(&b""[..], Value::ValueMaterialLibraryName("Material.Library".to_string())), parse_material_library_name_value(test_case_1));
        assert_nom_error!(parse_material_library_name_value(test_case_2));
        assert_nom_error!(parse_material_library_name_value(test_case_3));
    }

    #[test]
    fn parse_use_material_value_should_parse_properly() {
        let test_case_1 = &b" usemtl Material.01
"[..];
        let test_case_2 = &b" mtllib Material.01 after
"[..];
        let test_case_3 = &b" before mtllib Material.01
"[..];

        assert_eq!(Done(&b""[..], Value::ValueUseMaterialName("Material.01".to_string())), parse_use_material_value(test_case_1));
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

        assert_eq!(Done(&b""[..], Value::ValueVertexGeometric(Vertex{x: 1.0, y: 2.0, z: -3.0})), parse_vertex_geometry_value(test_case_1));
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

        assert_eq!(Done(&b""[..], Value::ValueVertexNormal(Vertex{x: 1.0, y: 2.0, z: -3.0})), parse_vertex_normal_value(test_case_1));
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

        let test_case_expected = Value::ValueTriangle(
            TriangleOfVertexTriplets{
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

        let test_case_expected = Value::ValueTriangle(
            TriangleOfVertexTriplets{
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

        let test_case_expected = Value::ValueTriangle(
            TriangleOfVertexTriplets{
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

        let test_case_expected = Value::ValueTriangle(
            TriangleOfVertexTriplets{
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
    fn parse_ValueGroup_value_should_parse_properly() {
        let test_case_1 = &b" g ValueGroup.1 ValueGroup.2 MaterialValueGroup.3.Yes!
"[..];
        let test_case_2 = &b" g ValueGroup.1
"[..];
        let test_case_3 = &b" before g ValueGroup1
"[..];

        assert_eq!(Done(&b""[..], Value::ValueGroup(vec!["ValueGroup.1".to_string(), "ValueGroup.2".to_string(), "MaterialValueGroup.3.Yes!".to_string()])), parse_ValueGroup_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::ValueGroup(vec!["ValueGroup.1".to_string()])), parse_ValueGroup_value(test_case_2));
        assert_nom_error!(parse_vertex_normal_value(test_case_3));
    }

    #[test]
    fn parse_smoothing_ValueGroup_value_should_parse_properly() {
        let test_case_1 = &b" s 0
"[..];
        let test_case_2 = &b" s 1
"[..];
        let test_case_3 = &b" before s 0
"[..];
        let test_case_4 = &b" s 0 1
"[..];

        assert_eq!(Done(&b""[..], Value::ValueSmoothingValueGroup(0)), parse_smoothing_ValueGroup_value(test_case_1));
        assert_eq!(Done(&b""[..], Value::ValueSmoothingValueGroup(1)), parse_smoothing_ValueGroup_value(test_case_2));
        assert_nom_error!(parse_vertex_normal_value(test_case_3));
        assert_nom_error!(parse_vertex_normal_value(test_case_4));
    }
}
