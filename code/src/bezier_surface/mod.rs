use crate::rendering::render::Vertex;


pub fn create_surface(size: usize) -> Vec<Vertex>{
    //Lägg till assert så den kallar att det är en pow av två
    let mut surface_points: Vec<Vertex> = Vec::with_capacity(size*size);
    let half_size:isize = (size/2) as isize;
    let start:isize = 0 - half_size;
    for i in start..half_size{
        for j in start..half_size{
            println!("Point is: {:#?}", (j, 0.0, i));
            surface_points.push(Vertex{position: [j as f32, 0.0, i as f32], normal: [0.0,0.0,0.0], tex_coords: [0.0, 1.0]});
        }
    }
    return surface_points;
}