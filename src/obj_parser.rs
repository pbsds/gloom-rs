use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::{self, Split};

const VALUE_SEPERATOR: &str = " ";
const INDEX_SEPERATOR: &str = "/";
const NAME_PREFIX: &str = "o ";

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
    fn from_extension(extension: &str) -> AttributeType {
        match extension {
            "v" => AttributeType::Vertex,
            "vp" => AttributeType::ParameterSpace,
            "f" => AttributeType::Indices,
            "l" => AttributeType::Line,
            "mtllib" => AttributeType::MaterialPath,
            "usemtl" => AttributeType::Material,
            "o" => AttributeType::Name,
            "g" => AttributeType::Group,
            "s" => AttributeType::Smoothing,
            _ => AttributeType::Unknown,
        }
    }
}

// Make public fields readonly once we're allowed to use other crates
pub struct Parser<'a> {
    pub model_path: &'a Path,
    contents: Option<Vec<String>>,
    pub vertices: Vec<Vec<f32>>,
    pub dimensions: i32,
    pub name: String,
}

impl Parser<'_> {
    pub fn new(path: &String) -> Parser {
        Parser {
            model_path: Path::new(path),
            contents: None,
            vertices: vec![],
            dimensions: 0,
            name: String::from("Untitled"),
        }
    }

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn parse(&mut self) {
        if let Ok(lines) = Self::read_lines(self.model_path) {
            lines.for_each(|line| {
                if let Ok(attribute) = line {
                    let mut attribute_parts = attribute.split(VALUE_SEPERATOR);
                    match AttributeType::from_extension(attribute_parts.next().unwrap()) {
                        AttributeType::Vertex => self.handle_vertex(attribute_parts),
                        AttributeType::ParameterSpace => todo!(),
                        AttributeType::Indices => todo!(),
                        AttributeType::Line => todo!(),
                        AttributeType::Name => self.name = self.handle_text_attribute(attribute, String::from(NAME_PREFIX)),
                        AttributeType::Group => todo!(),
                        AttributeType::Smoothing => todo!(),
                        AttributeType::Material => todo!(),
                        AttributeType::MaterialPath => todo!(),
                        AttributeType::Unknown => self.handle_comment(attribute_parts),
                    }
                }
            });
        }
    }

    fn handle_vertex(&mut self, data: str::Split<&str>) {
        let mut dimensions = 0;
        let mut vertex: Vec<f32> = vec![];
        data.for_each(|coordinate| {
            let parsed_coordinate = coordinate.parse::<f32>();
            match parsed_coordinate {
                Ok(coord) => vertex.push(coord),
                Err(e) => {
                    println!("Unable to parse, vertex dropped: {}", e);
                    return;
                },
            }
            dimensions += 1;
        });
        self.vertices.push(vertex);
        self.dimensions = dimensions;
    }

    fn handle_text_attribute(&mut self, data: String, prefix: String) -> String {
        return data.replace(&prefix, &data);
    }

    fn handle_comment(&mut self, data: str::Split<&str>) {
        print!("#");
        data.for_each(|word| {
            print!("{}", word);
        });
        println!();
    }
}
