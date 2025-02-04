use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};



pub fn ray_plane_intersect(p0: Vec3,d:Vec3,q:Vec3,n:Vec3) -> Vec3{

    /*
    P0: The starting point of the ray 
    d: The direction vector of the ray 
    Q: A point on the plane 
    n: The normal vector of the plane 

    */

    let denom = n.dot(d);

    let t = (n.dot(q-p0)) / denom;

    let intersection_point = p0 + t * d;

    return intersection_point;
}

pub fn ndc_to_intersection(mouse_ndc: &Vec3, camera_matrix: &Mat4, camera_pos: Vec3, perspective_mat: &Mat4) -> Vec3{
                    //let inverse = Mat4::inverse(&(Mat4::from_cols_array_2d(&perspective)*camera_matrix*Mat4::from_cols_array_2d(&hex_size_mat)));
    let eyespace = Mat4::inverse(perspective_mat);
    let eye_space_vector = Vec4::new(mouse_ndc.x, mouse_ndc.y, -1.0, 1.0);
    let mut eye_vector = eyespace*eye_space_vector;
    eye_vector.z = -1.0;
    eye_vector.w = 0.0;
    let worldspace = Mat4::inverse(camera_matrix);
    let world_vector:Vec4 = worldspace*eye_vector;
    let norm_world:Vec3 = world_vector.xyz().normalize();
    ray_plane_intersect(Vec3::new(-camera_pos.x,-camera_pos.y,camera_pos.z), norm_world, Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,1.0))
}