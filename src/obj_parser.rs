use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::{self, Split};


const VALUE_SEPERATOR: &str = " ";
const INDEX_SEPERATOR: &str = "/";
const NAME_PREFIX: &str = "o ";
const GROUP_PREFIX: &str = "g ";
const SMOOTH_PREFIX: &str = "s ";
const MATERIAL_PREFIX: &str = "usemtl ";
const MATERIAL_PATH_PREFIX: &str = "mtllib ";

pub enum AttributeType {
    Vertex,
    Textures,
    Normals,
    ParameterSpace,
    Faces,
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
            "vt" => AttributeType::Textures,
            "vn" => AttributeType::Normals,
            "vp" => AttributeType::ParameterSpace,
            "f" => AttributeType::Faces,
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

pub struct Face {
    pub vertices: Vec<u32>,
    pub textures: Vec<Option<u32>>,
    pub normals: Vec<Option<u32>>,
}

impl Face {
    pub fn new(vertices: Vec<u32>, textures: Vec<Option<u32>>, normals: Vec<Option<u32>>) -> Face {
        Face {
            vertices,
            textures,
            normals,
        }
    }

    pub fn points(&self) -> u32 {
        return self.vertices.len() as u32;
    }
}

// Make public fields readonly once we're allowed to use other crates
pub struct Parser<'a> {
    pub model_path: &'a Path,
    pub vertices: Vec<Vec<f32>>,
    pub name: String,
    pub group: String,
    pub material: String,
    pub material_path: String,
    pub smoothing_group: i32,
    pub faces: Vec<Face>,
}

impl Parser<'_> {
    pub fn new(path: &String) -> Parser {
        Parser {
            model_path: Path::new(path),
            vertices: vec![],
            name: String::from("Untitled"),
            group: String::from(""),
            material: String::from(""),
            material_path: String::from(""),
            smoothing_group: 0,
            faces: vec![],
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
                        AttributeType::ParameterSpace => self.handle_parameter_space(attribute_parts),
                        AttributeType::Faces => self.handle_face(attribute_parts),
                        AttributeType::Line => self.handle_line(attribute_parts),
                        AttributeType::Name => self.name = self.handle_text_attribute(attribute, String::from(NAME_PREFIX)),
                        AttributeType::Group => self.group = self.handle_text_attribute(attribute, String::from(GROUP_PREFIX)),
                        AttributeType::Smoothing => self.handle_smoothing(attribute, String::from(SMOOTH_PREFIX)),
                        AttributeType::Material => self.material = self.handle_text_attribute(attribute, String::from(MATERIAL_PREFIX)),
                        AttributeType::MaterialPath => self.material_path = self.handle_text_attribute(attribute, String::from(MATERIAL_PATH_PREFIX)),
                        AttributeType::Unknown => self.handle_comment(attribute_parts),
                        AttributeType::Textures => self.handle_texture(attribute_parts),
                        AttributeType::Normals => self.handle_normal(attribute_parts),
                    }
                }
            });
        }
    }

    pub fn vertex_indices(&mut self) -> Vec<u32> {
        let mut vertices: Vec<u32> = vec![];
        for face in &mut self.faces {
            vertices.append(&mut face.vertices);
        }
        return vertices
    }

    pub fn flat_vertices(&mut self) -> Vec<f32> {
        return self.vertices.clone().into_iter().flatten().collect();
    }

    pub fn nonhomogenous_vertices(&mut self) -> Vec<f32> {
        let mut vertices: Vec<f32> = vec![];
        let mut counter = 0;
        for vertex in self.flat_vertices() {
            counter += 1;
            counter %= 4;
            if counter == 0 {
                continue;
            }
            vertices.push(vertex);
        }
        return vertices;
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

        if dimensions == 3 {
            vertex.push(1.0);
        }
        self.vertices.push(vertex);
    }

    fn handle_texture(&mut self, data: str::Split<&str>) {
        println!("Textures not implemented!");
    }

    fn handle_normal(&mut self, data: str::Split<&str>) {
        println!("Normals not implemented!");
    }

    fn handle_parameter_space(&mut self, data: str::Split<&str>) {
        println!("Parameter space not implemented!");
    }

    fn handle_line(&mut self, data: str::Split<&str>) {
        println!("Lines not implemented!");
    }

    fn handle_smoothing(&mut self, data: String, prefix: String) {
        let stripped_data = data.replace(&prefix, &data);
        if stripped_data == *"off" {
            self.smoothing_group = 0;
        } else {
            let group = stripped_data.parse::<i32>();
            if let Ok(group_number) = group {
                self.smoothing_group = group_number;
            } else {
                self.smoothing_group = 0;
            }
        }
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

    fn handle_face(&mut self, data: str::Split<&str>) {
        let mut vertices: Vec<u32> = vec![];
        let mut textures: Vec<Option<u32>> = vec![];
        let mut normals: Vec<Option<u32>> = vec![];
        data.for_each(|index| {
            let mut elements = index.split(INDEX_SEPERATOR);
            
            let vertex = elements.next().unwrap().parse::<u32>();
            if let Ok(parsed_vertex) = vertex {
                vertices.push(parsed_vertex - 1);
            }

            match elements.next() {
                Some(t) => {
                    if let Ok(texture) = t.parse::<u32>() {
                        textures.push(Some(texture - 1));
                    } else {
                        textures.push(None);
                    }
                },
                None => textures.push(None),
            }

            match elements.next() {
                Some(n) => {
                    if let Ok(normal) = n.parse::<u32>() {
                        normals.push(Some(normal - 1));
                    } else {
                        normals.push(None);
                    }
                },
                None => normals.push(None),
            }
        });
        self.faces.push(Face::new(vertices, textures, normals));
    }
}
