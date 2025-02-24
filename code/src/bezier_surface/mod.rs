use glam::Vec3;

use crate::rendering::render::{Vertex, VertexSimple};

pub struct Surface{
    start_pos: Vec3,
    num_quads_x: usize,
    num_quads_z: usize,
    step_size_x: f32,
    step_size_z: f32,
    pub points: Vec<VertexSimple>,
    pub inds: Vec<u16>
}

impl Surface{

    pub fn new(
        start_pos: Vec3,
        step_size_x: f32,
        step_size_z: f32,
        num_quads_x: usize,
        num_quads_z: usize,
    ) -> Surface{
        let (points, inds) = create_surface(start_pos, step_size_x, step_size_z, num_quads_x, num_quads_z);
        Surface{
            start_pos: start_pos,
            num_quads_x: num_quads_x,
            num_quads_z: num_quads_z,
            step_size_x: step_size_x,
            step_size_z: step_size_z,
            points: points,
            inds: inds
        }
    }

    pub fn get_grass_posistions(&self, grass_per_dim_quad: usize) -> Vec<VertexSimple>{
        let mut samples = Vec::new();
        // Total vertices per row in the full grid.
        let total_vertices_x = self.num_quads_x * 3 + 1;
        
        // Loop through each quad in the grid.
        for quad_z in 0..self.num_quads_z {
            for quad_x in 0..self.num_quads_x {
                // For each quad, compute its starting grid coordinates.
                let start_x = quad_x * 3;
                let start_z = quad_z * 3;
                
                // Extract the 16 control points (4×4 block) for the current quad.
                let mut quad_vertices = Vec::with_capacity(16);
                for i in 0..4 {
                    for j in 0..4 {
                        let index = (start_z + i) * total_vertices_x + (start_x + j);
                        quad_vertices.push(Vec3::from_array(self.points[index].w_position));
                    }
                }
                
                // Now, sample the quad. u and v vary between 0.0 and 1.0.
                // If there is only one sample, use 0.0; otherwise, space them evenly.
                for v_idx in 0..grass_per_dim_quad {
                    let v = if grass_per_dim_quad > 1 {
                        v_idx as f32 / (grass_per_dim_quad - 1) as f32
                    } else {
                        0.0
                    };
                    for u_idx in 0..grass_per_dim_quad {
                        let u = if grass_per_dim_quad > 1 {
                            u_idx as f32 / (grass_per_dim_quad - 1) as f32
                        } else {
                            0.0
                        };

                        let (bu, _) = Self::bernstain(u);
                        let (bv, _) = Self::bernstain(v);
                        
                        let ev_pos:Vec3 =       quad_vertices[0]*bu[0]*bv[0] + quad_vertices[1]*bu[0]*bv[1] + quad_vertices[2]*bu[0]*bv[2] + quad_vertices[3]*bu[0]*bv[3] + 
                                                quad_vertices[4]*bu[1]*bv[0] + quad_vertices[5]*bu[1]*bv[1] + quad_vertices[6]*bu[1]*bv[2] + quad_vertices[7]*bu[1]*bv[3] + 
                                                quad_vertices[8]*bu[2]*bv[0] + quad_vertices[9]*bu[2]*bv[1] + quad_vertices[10]*bu[2]*bv[2] + quad_vertices[11]*bu[2]*bv[3] + 
                                                quad_vertices[12]*bu[3]*bv[0] + quad_vertices[13]*bu[3]*bv[1] + quad_vertices[14]*bu[3]*bv[2] + quad_vertices[15]*bu[3]*bv[3];
                        
                        samples.push(VertexSimple {
                            w_position: [ev_pos.x,ev_pos.y,ev_pos.z]
                        });
                    }
                }
            }
        }
        
        samples
    } 


    pub fn evaluate(&self, pos: Vec3) -> Option<(Vec3,Vec3,Vec3)>{

        let bez_in = self.get_points(pos);

        if bez_in.is_none(){
            return None
        }

        let (points, (u,v)) = bez_in.unwrap();
        //println!("U and V is: {}, {}", u, v);
        //println!("Points are: {:#?}", points);
        let (bu, dbu) = Self::bernstain(u);
        let (bv, dbv) = Self::bernstain(v);
        
        let ev_pos = points[0]*bu[0]*bv[0] + points[1]*bu[0]*bv[1] + points[2]*bu[0]*bv[2] + points[3]*bu[0]*bv[3] + 
                                points[4]*bu[1]*bv[0] + points[5]*bu[1]*bv[1] + points[6]*bu[1]*bv[2] + points[7]*bu[1]*bv[3] + 
                                points[8]*bu[2]*bv[0] + points[9]*bu[2]*bv[1] + points[10]*bu[2]*bv[2] + points[11]*bu[2]*bv[3] + 
                                points[12]*bu[3]*bv[0] + points[13]*bu[3]*bv[1] + points[14]*bu[3]*bv[2] + points[15]*bu[3]*bv[3];

        let dPos_du =         points[0]*dbu[0]*bv[0] + points[1]*dbu[0]*bv[1] + points[2]*dbu[0]*bv[2] + points[3]*dbu[0]*bv[3] + 
                                    points[4]*dbu[1]*bv[0] + points[5]*dbu[1]*bv[1] + points[6]*dbu[1]*bv[2] + points[7]*dbu[1]*bv[3] + 
                                    points[8]*dbu[2]*bv[0] + points[9]*dbu[2]*bv[1] + points[10]*dbu[2]*bv[2] + points[11]*dbu[2]*bv[3] + 
                                    points[12]*dbu[3]*bv[0] + points[13]*dbu[3]*bv[1] + points[14]*dbu[3]*bv[2] + points[15]*dbu[3]*bv[3];

        let dPos_dv =         points[0]*bu[0]*dbv[0] + points[1]*bu[0]*dbv[1] + points[2]*bu[0]*dbv[2] + points[3]*bu[0]*dbv[3] + 
                                    points[4]*bu[1]*dbv[0] + points[5]*bu[1]*dbv[1] + points[6]*bu[1]*dbv[2] + points[7]*bu[1]*dbv[3] + 
                                    points[8]*bu[2]*dbv[0] + points[9]*bu[2]*dbv[1] + points[10]*bu[2]*dbv[2] + points[11]*bu[2]*dbv[3] + 
                                    points[12]*bu[3]*dbv[0] + points[13]*bu[3]*dbv[1] + points[14]*bu[3]*dbv[2] + points[15]*bu[3]*dbv[3];                        
        return Some((ev_pos, dPos_du, dPos_dv))
    }

    fn get_points(&self, pos: Vec3)-> Option<(Vec<Vec3>, (f32,f32))>{
        let grid_pos = pos - self.start_pos;
        if grid_pos.x < 0.0 || grid_pos.z < 0.0{
            //println!("Point is too far away!");
            return None;
        }   
        let quad_width = 3.0 * self.step_size_x;
        let quad_depth = 3.0 * self.step_size_z;

        let quad_index_x = (grid_pos.x  / quad_width).floor() as usize;
        let quad_index_z = (grid_pos.z / quad_depth).floor() as usize;

        if quad_index_x >= self.num_quads_x || quad_index_z >= self.num_quads_z {
            //println!("Point is outside!");
            return None;
        }

        let start_x = quad_index_x * 3;
        let start_z = quad_index_z * 3;
        let total_vertices_x = self.num_quads_x * 3 + 1;

        // Compute how far into the quad we are in x and z directions (0.0 to 1.0)
        let u = (grid_pos.x % quad_width) / quad_width;
        let v = (grid_pos.z % quad_depth) / quad_depth;
        // Now collect the 16 vertices from the 4×4 block.
        let mut quad_vertices = Vec::with_capacity(16);
        for i in 0..4 {
            for j in 0..4 {
                let vertex_index = (start_z + i) * total_vertices_x + (start_x + j);
                // Extra check in case the index goes out of bounds.
                if vertex_index >= self.points.len() {
                    return None;
                }
                quad_vertices.push(Vec3::from_array(self.points[vertex_index].w_position));
            }
        }
        
        Some((quad_vertices, (u,v)))
    }

    //Precompute this...
    fn bernstain(t: f32)-> ([f32;4], [f32;4]){
        let mut b = [0.0;4];
        b[0] = (1.0 - t).powi(3);
        b[1] = 3.0 * (1.0 - t).powi(2) * t;
        b[2] = 3.0 * (1.0 - t) * t.powi(2);
        b[3] = t.powi(3);

        let mut db = [0.0;4];
        db[0] = -3.0*(1.0 - t).powi(2);
        db[1] = -6.0 * (1.0 - t) * t + 3.0 * (1.0 - t).powi(2);
        db[2] = -3.0 * (t).powi(2) + 6.0 * t * (1.0 - t);
        db[3] = 3.0 * (t).powi(2);

        return (b,db);
    }

}


pub fn create_surface_quad(start_pos: Vec3, size: Vec3) -> Vec<VertexSimple>{
    //Lägg till assert så den kallar att det är en pow av två
    let step_size_x = (size.x-start_pos.x).abs();
    let step_size_z = (size.z-start_pos.z).abs();
    let mut surface_points: Vec<VertexSimple> = Vec::with_capacity(16);
    for i in 0..4{
        for j in 0..4{
            surface_points.push(VertexSimple{w_position: [start_pos.x + step_size_x*j as f32, 0.0, start_pos.z + step_size_z*i as f32]});
        }
    }
    let add_vec= Vec3::new(0.0,0.0,4.0);
    surface_points.append(&mut create_connected_quad(start_pos+add_vec, size));
    return surface_points;
}

pub fn create_surface(
    start_pos: Vec3,
    step_size_x: f32,
    step_size_z: f32,
    num_quads_x: usize,
    num_quads_z: usize,
) -> (Vec<VertexSimple>, Vec<u16>) {
    // Calculate total vertices in each direction
    let total_vertices_x = num_quads_x * 3 + 1;
    let total_vertices_z = num_quads_z * 3 + 1;

    // Generate all vertices in a grid
    let mut vertices = Vec::with_capacity(total_vertices_x * total_vertices_z);
    for z in 0..total_vertices_z {
        for x in 0..total_vertices_x {
            let pos_x = start_pos.x + x as f32 * step_size_x;
            let pos_z = start_pos.z + z as f32 * step_size_z;
            vertices.push(VertexSimple {
                w_position: [pos_x, start_pos.y, pos_z],
            });
        }
    }

    // Generate indices for each quad (4x4 points per quad)
    let mut indices = Vec::with_capacity(num_quads_x * num_quads_z * 16);
    for quad_z in 0..num_quads_z {
        for quad_x in 0..num_quads_x {
            let start_x = quad_x * 3;
            let start_z = quad_z * 3;
            
            // Add indices for 4x4 grid in row-major order
            for i in 0..4 {
                for j in 0..4 {
                    let x = start_x + j;
                    let z = start_z + i;
                    indices.push((z * total_vertices_x + x) as u16);
                }
            }
        }
    }

    (vertices, indices)
}

fn create_connected_quad(start_pos: Vec3, size: Vec3) -> Vec<VertexSimple>{
    //Lägg till assert så den kallar att det är en pow av två
    let step_size_x = (size.x-start_pos.x).abs();
    let step_size_z = (size.z-start_pos.z).abs();
    let mut surface_points: Vec<VertexSimple> = Vec::with_capacity(16);
    for i in 0..4{
        for j in 0..4{
            surface_points.push(VertexSimple{w_position: [start_pos.x + step_size_x*j as f32, 0.0, start_pos.z + step_size_z*i as f32]});
        }
    }

    return surface_points;
}