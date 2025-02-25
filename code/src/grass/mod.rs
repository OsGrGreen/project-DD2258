use crate::rendering::render::Vertex;



pub fn get_grass_shape() -> (Vec<Vertex>, Vec<u16>){
    (vec![
        //Two slightly rotated quads
        Vertex { position: [0.3535, 0.5, -0.3535], normal: [0.707, 0.0, 0.707], tex_coords: [1.0, 1.0] }, // Top Right
        Vertex { position: [-0.3535, 0.5, 0.3535], normal: [0.707, 0.0, 0.707], tex_coords: [0.0, 1.0] },  // Top Left
        Vertex { position: [-0.3535, -0.5, 0.3535], normal: [0.707, 0.0, 0.707], tex_coords: [0.0, 0.0] }, // Bottom Left
        Vertex { position: [0.3535, -0.5, -0.3535], normal: [0.707, 0.0, 0.707], tex_coords: [1.0, 0.0] },  // Bottom Right

        Vertex { position: [0.3535, 0.5, 0.3535], normal: [0.707, 0.0, -0.707], tex_coords: [1.0, 1.0] },  // Top Front
        Vertex { position: [-0.3535, 0.5, -0.3535], normal: [0.707, 0.0, -0.707], tex_coords: [0.0, 1.0] }, // Top Back
        Vertex { position: [-0.3535, -0.5, -0.3535], normal: [0.707, 0.0, -0.707], tex_coords: [0.0, 0.0] }, // Bottom Back
        Vertex { position: [0.3535, -0.5, 0.3535], normal: [0.707, 0.0, -0.707], tex_coords: [1.0, 0.0] },  // Bottom Front
    ],
    vec![0, 1, 2,   
        0, 2, 3,   

        4, 5, 6, 
        4, 6, 7,  
        ])
}