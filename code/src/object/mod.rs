use glam::{Mat4, Vec3};
pub mod point;

#[derive(Copy, Clone,Debug)]
pub struct WorldObject{
    scaling: Mat4,
    rotation: Mat4,
    translation: Mat4,
    pub velocity: Vec3,
}

impl WorldObject{
    pub fn new() -> WorldObject{
        WorldObject{
            scaling: Mat4::IDENTITY,
            rotation: Mat4::IDENTITY,
            translation: Mat4::IDENTITY,
            velocity: Vec3::new(0.0, -2.0, 0.0),
        }
    }
    pub fn new_from_pos(initial_pos: Vec3) -> WorldObject{
        WorldObject{
            scaling: Mat4::IDENTITY,
            rotation: Mat4::IDENTITY,
            translation: Mat4::from_translation(initial_pos),
            velocity: Vec3::new(0.0, -2.0, 0.0),
        }
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f32){
        let rotation_matrix = Mat4::from_axis_angle(axis, angle);
        self.rotation *= rotation_matrix;
    }

    pub fn scale(&mut self, scale: Vec3){
        let scale_matrix = Mat4::from_scale(scale);
        self.scaling *= scale_matrix;
    }

    pub fn translate(&mut self, target: Vec3){
        let translation_matrix = Mat4::from_translation(target);
        self.translation *= translation_matrix;
    }

    pub fn set_translation(&mut self, target: Vec3){
        self.translation = Mat4::from_translation(target);
    }

    pub fn get_posistion(&self) -> Vec3{
        let translation = self.translation.col(3);
        Vec3::new(translation.x, translation.y,translation.z)
    }


    pub fn get_model(self) -> Mat4{
        return self.translation*self.scaling*self.rotation
    }
}