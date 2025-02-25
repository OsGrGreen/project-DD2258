use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use winit::dpi::PhysicalSize;


//For all of these functions the following idea holds:
/*Basically just make the reverse of the camera and perspective to translate screen pos to world pos */


//Is this even accurate?
pub fn distance_ray_point(ray_origin: Vec3, ray_direction: Vec3, point: Vec3) -> f32{
    let point_to_origin = point - ray_origin;
    let cross_product = ray_direction.cross(point_to_origin);
    cross_product.length()
}

pub fn ndc_to_direction(
    mouse_ndc: &Vec3, 
    camera_matrix: &Mat4, 
    perspective_mat: &Mat4
) -> Vec3 {
    let eyespace = Mat4::inverse(perspective_mat);
    let eye_space_vector = Vec4::new(mouse_ndc.x, mouse_ndc.y, -1.0, 1.0);

    let mut eye_vector = eyespace * eye_space_vector;
    eye_vector.z = -1.0;
    eye_vector.w = 0.0;

    let worldspace = Mat4::inverse(camera_matrix);
    let world_vector: Vec4 = worldspace * eye_vector;
    world_vector.xyz().normalize()
}

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

