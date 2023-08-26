use std::{
    str,
    path::Path,
};

pub enum AttributeType {
    Vertex,
    ParameterSpaceVertices,
    Face,
    Indices,
    Line,
    Name,
    Group,
    Smoothing,
    Material,
    MaterialPath,
    Unknown,
}

impl Parser {

}

impl AttributeType {
    fn from_ext(ext: &std::ffi::OsStr) -> Result<AttributeType, String> {
        match ext.to_str().expect("Unkown attribute or comment") {
            "v" => { Ok{AttributeType::Vertex} },
            e: => { Ok{AttributeType::Unknown} },
        }
    }
}