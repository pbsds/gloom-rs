use std::{path::Path, str};

pub enum AttributeType {
    Vertex,
    ParameterSpace,
    Indices,
    Line,
    Name,
    Group,
    Smoothing,
    Material,
    MaterialPath,
    Unknown,
}

impl AttributeType {
    fn from_extension(ext: &std::ffi::OsStr) -> Result<AttributeType, String> {
        match ext.to_str().expect("Unkown attribute or comment") {
            "v" => Ok(AttributeType::Vertex),
            "vp" => Ok(AttributeType::ParameterSpace),
            "f" => Ok(AttributeType::Indices),
            "l" => Ok(AttributeType::Line),
            "mtllib" => Ok(AttributeType::MaterialPath),
            "usemtl" => Ok(AttributeType::Material),
            "o" => Ok(AttributeType::Name),
            "g" => Ok(AttributeType::Group),
            "s" => Ok(AttributeType::Smoothing),
            _ => Ok(AttributeType::Unknown),
        }
    }
}

pub struct Parser<'a> {
    pub model_path: &'a Path,
}

impl Parser<'_> {
    pub fn new(path: &String) -> Parser {
        Parser {
            model_path: Path::new(path),
        }
    }

    pub fn parse(&self) {
        println!("{}", self.model_path.display());
    }
}
