use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use winit::dpi::PhysicalSize;



pub fn world_to_pixel(
    world_point: Vec3, 
    camera_matrix: &Mat4, 
    window_size: PhysicalSize<u32>, 
    perspective_mat: &Mat4
) -> Vec2 {
    let projection_view = *perspective_mat * *camera_matrix;
    let resulting = projection_view*Vec4::new(world_point.x, world_point.y, world_point.z, 0.0);
    Vec2::new(resulting.x, resulting.y)
} 

pub fn ray_plane_intersect(p0: Vec3, d: Vec3, q: Vec3, n: Vec3) -> Vec3 {
    /*
    P0: The starting point of the ray 
    d: The direction vector of the ray 
    Q: A point on the plane 
    n: The normal vector of the plane 
    */

    let denom = n.dot(d);

    let t = (n.dot(q - p0)) / denom;

    let intersection_point = p0 + t * d;

    intersection_point
}

pub fn ndc_to_intersection(
    mouse_ndc: &Vec3, 
    camera_matrix: &Mat4, 
    camera_pos: Vec3, 
    perspective_mat: &Mat4
) -> Vec3 {
    let eyespace = Mat4::inverse(perspective_mat);
    let eye_space_vector = Vec4::new(mouse_ndc.x, mouse_ndc.y, -1.0, 1.0);

    let mut eye_vector = eyespace * eye_space_vector;
    eye_vector.z = -1.0;
    eye_vector.w = 0.0;

    let worldspace = Mat4::inverse(camera_matrix);
    let world_vector: Vec4 = worldspace * eye_vector;
    let norm_world: Vec3 = world_vector.xyz().normalize();

    ray_plane_intersect(
        Vec3::new(camera_pos.x, camera_pos.y, camera_pos.z), 
        norm_world, 
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(0.0, 0.0, -1.0)  
    )
}

pub fn ndc_to_point(
    mouse_ndc: &Vec3, 
    camera_matrix: &Mat4, 
    camera_pos: Vec3, 
    perspective_mat: &Mat4,
    dist: f32,
) -> Vec3 {
    let eyespace = Mat4::inverse(perspective_mat);
    let eye_space_vector = Vec4::new(mouse_ndc.x, mouse_ndc.y, -1.0, 1.0);

    let mut eye_vector = eyespace * eye_space_vector;
    eye_vector.z = -1.0;
    eye_vector.w = 0.0;

    let worldspace = Mat4::inverse(camera_matrix);
    let world_vector: Vec4 = worldspace * eye_vector;
    let norm_world: Vec3 = world_vector.xyz().normalize();

    camera_pos - norm_world * dist
}

