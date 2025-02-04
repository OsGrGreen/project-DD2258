use std::fs;
use std::fs::File;
use crate::rendering::render::Vertex;

pub mod input_handler;
pub mod ray_library;

pub fn read_shader(file_path: &str) -> String{

    let contents = fs::read_to_string(file_path)
        .expect("Was not able to read shader");

    return contents
}

pub fn read_model(file_path: &str) -> Vec<Vertex>{

    let file = File::open(file_path).expect("Could not find model file");
    let mut data = ::std::io::BufReader::new(file);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data:Vec<Vertex> = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position: [f32; 3] = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let _texture = texture.unwrap_or([0.0, 0.0]);
                        let _normal = normal.unwrap_or([0.0, 0.0, 0.0]);
                        
                        vertex_data.push(Vertex{
                            position:position,
                            normal: _normal,
                            tex_coords: _texture,
                        })
                    }
                },
            }
        }
    }
    vertex_data
}