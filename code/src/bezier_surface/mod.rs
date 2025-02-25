use glam::Vec3;
use rand::Rng;

use crate::rendering::render::{Vertex, VertexSimple};

#[derive(Copy, Clone,Debug)]
pub struct GrassVertex {
    pub g_position: [f32; 3],
    pub g_normal: [f32; 3],
}
implement_vertex!(GrassVertex, g_position, g_normal);


pub struct Surface{
    pub start_pos: Vec3,
    pub num_quads_x: usize,
    pub num_quads_z: usize,
    pub step_size_x: f32,
    pub step_size_z: f32,
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

    pub fn get_grass_posistions(&self, grass_per_dim_quad: usize) -> Vec<GrassVertex>{
        let mut samples = Vec::new();
        // Im sure there must be a better way to do this...
        // But once again it seems to work
        // And the deadline was getting pretty close
        let total_vertices_x = self.num_quads_x * 3 + 1;
        //Previously added random offset to each posistion
        //However, this will never make the grass have the same posistion twice
        //Which looks bad when updating the grass many times in a row
        let mut rng = rand::rng();
        for quad_z in 0..self.num_quads_z {
            for quad_x in 0..self.num_quads_x {
                let start_x = quad_x * 3;
                let start_z = quad_z * 3;
                
                let mut quad_vertices = Vec::with_capacity(16);
                for i in 0..4 {
                    for j in 0..4 {
                        let index = (start_z + i) * total_vertices_x + (start_x + j);
                        quad_vertices.push(Vec3::from_array(self.points[index].w_position));
                    }
                }
                
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

                        let (bu, du) = Self::bernstain(u);
                        let (bv, dv) = Self::bernstain(v);
                        
                        let (pos, du_vec, dv_vec) = Self::get(&quad_vertices,bu,du,bv,dv);


                        samples.push(GrassVertex {
                            g_position: (pos).to_array(),
                            g_normal: du_vec.cross(dv_vec).to_array(),
                        });
                    }
                }
            }
        }
        
        samples
    } 


    pub fn evaluate(&self, pos: Vec3) -> Option<(Vec3,Vec3,Vec3)>{

        let bez_in = self.get_quad_data(pos);
        
        if bez_in.is_none(){
            return None
        }

        let (points, (u,v)) = bez_in.unwrap();
        let (bu, dbu) = Self::bernstain(u);
        let (bv, dbv) = Self::bernstain(v);
        
                      
        return Some(Self::get(&points,bu,dbu,bv,dbv))
    }

    fn get(points: &Vec<Vec3>, bu: [f32;4], dbu: [f32;4], bv: [f32;4], dbv: [f32;4]) -> (Vec3,Vec3,Vec3){
        let ev_pos = points[0]*bu[0]*bv[0] + points[4]*bu[0]*bv[1] + points[8]*bu[0]*bv[2] + points[12]*bu[0]*bv[3] + 
                            points[1]*bu[1]*bv[0] + points[5]*bu[1]*bv[1] + points[9]*bu[1]*bv[2] + points[13]*bu[1]*bv[3] + 
                            points[2]*bu[2]*bv[0] + points[6]*bu[2]*bv[1] + points[10]*bu[2]*bv[2] + points[14]*bu[2]*bv[3] + 
                            points[3]*bu[3]*bv[0] + points[7]*bu[3]*bv[1] + points[11]*bu[3]*bv[2] + points[15]*bu[3]*bv[3];

        let d_pos_du =         points[0]*dbu[0]*bv[0] + points[4]*dbu[0]*bv[1] + points[8]*dbu[0]*bv[2] + points[12]*dbu[0]*bv[3] + 
                                    points[1]*dbu[1]*bv[0] + points[5]*dbu[1]*bv[1] + points[9]*dbu[1]*bv[2] + points[13]*dbu[1]*bv[3] + 
                                    points[2]*dbu[2]*bv[0] + points[6]*dbu[2]*bv[1] + points[10]*dbu[2]*bv[2] + points[14]*dbu[2]*bv[3] + 
                                    points[3]*dbu[3]*bv[0] + points[7]*dbu[3]*bv[1] + points[11]*dbu[3]*bv[2] + points[15]*dbu[3]*bv[3];

        let d_pos_dv =         points[0]*bu[0]*dbv[0] + points[4]*bu[0]*dbv[1] + points[8]*bu[0]*dbv[2] + points[12]*bu[0]*dbv[3] + 
                                    points[1]*bu[1]*dbv[0] + points[5]*bu[1]*dbv[1] + points[9]*bu[1]*dbv[2] + points[13]*bu[1]*dbv[3] + 
                                    points[2]*bu[2]*dbv[0] + points[6]*bu[2]*dbv[1] + points[10]*bu[2]*dbv[2] + points[14]*bu[2]*dbv[3] + 
                                    points[3]*bu[3]*dbv[0] + points[7]*bu[3]*dbv[1] + points[11]*bu[3]*dbv[2] + points[15]*bu[3]*dbv[3];  
        return (ev_pos, d_pos_du, d_pos_dv)
    }

    fn get_quad_data(&self, point: Vec3) -> Option<(Vec<Vec3>, (f32, f32))> {

        let total_vertices_x = self.num_quads_x * 3 + 1;
        let quad_width = 3.0 * self.step_size_x;
        let quad_depth = 3.0 * self.step_size_z;


    
        let dx = point.x - self.start_pos.x;
        let dz = point.z - self.start_pos.z;

        let total_width = self.num_quads_x as f32 * quad_width;
        let total_depth = self.num_quads_z as f32 * quad_depth;
        if dx < 0.0 || dz < 0.0 || dx > total_width || dz > total_depth {
            return None;
        }

        let quad_x = (dx / quad_width).floor() as usize;
        let quad_z = (dz / quad_depth).floor() as usize;


        let local_x = dx - (quad_x as f32 * quad_width);
        let local_z = dz - (quad_z as f32 * quad_depth);
        let u = local_x / quad_width;
        let v = local_z / quad_depth;
    
        let mut control_points = Vec::with_capacity(16);
        for v_idx in 0..4 {
            for u_idx in 0..4 {
                let grid_x = quad_x * 3 + u_idx; 
                let grid_z = quad_z * 3 + v_idx; 
                let index = grid_z * total_vertices_x + grid_x;
                if index >= self.points.len() {
                    return None;
                }
                control_points.push(Vec3::from_array(self.points[index].w_position));
            }
        }

        Some((control_points, (u, v)))
    }

    //Precompute this...
    //Atleast for the grass or something
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



pub fn create_surface(
    start_pos: Vec3,
    step_size_x: f32,
    step_size_z: f32,
    num_quads_x: usize,
    num_quads_z: usize,
) -> (Vec<VertexSimple>, Vec<u16>) {
    let total_vertices_x = num_quads_x * 3 + 1;
    let total_vertices_z = num_quads_z * 3 + 1;

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

    let mut indices = Vec::with_capacity(num_quads_x * num_quads_z * 16);
    for quad_z in 0..num_quads_z {
        for quad_x in 0..num_quads_x {
            let start_x = quad_x * 3;
            let start_z = quad_z * 3;
            
            // Row-major order
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
