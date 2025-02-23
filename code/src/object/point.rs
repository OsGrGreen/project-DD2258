use glam::{Mat4, Vec2, Vec3};

use super::WorldObject;
#[derive(Copy, Clone,Debug)]

pub struct WorldPoint{
    obj: WorldObject,
    radius: f32,
    center: Vec2,
}

impl WorldPoint{
    pub fn new(radius: f32, center: Vec2, pos: Vec3) -> WorldPoint{
        WorldPoint{
            obj: WorldObject::new_from_pos(pos),
            radius: radius,
            center: center,
        }
    }

    pub fn get_model(self) -> WorldObject{
        self.obj
    }
    
    pub fn get_radius(self) -> f32{
        self.radius
    }

    pub fn get_center(self) -> Vec2{
        self.center
    }

    pub fn get_mut_model(&mut self) -> &mut WorldObject{
        &mut self.obj
    }
}