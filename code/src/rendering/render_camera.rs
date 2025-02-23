
use glam::{Mat4, Vec3, Vec4};


pub struct RenderCamera{
    camera_pos: Vec3,
    camera_prev_pos: Vec3,
    camera_target: Vec3,
    camera_up: Vec3,
    camera_front: Vec3,
    pub perspective: Mat4,
    pub camera_matrix: Mat4,
}

const SQRT3:f32 = 1.7320508;

impl RenderCamera{

    pub fn new(start_pos: Vec3, target:Vec3, up:Vec3, front:Vec3) -> RenderCamera{

        RenderCamera{camera_pos:start_pos, camera_prev_pos:start_pos,camera_target:target,camera_up:up, camera_front:front, perspective:Mat4::ZERO, camera_matrix: Mat4::ZERO}
    }

    pub fn get_pos(&self) -> Vec3{
        return self.camera_pos
    }

    pub fn get_movement_delta(&mut self) -> Vec3{
        let delta = self.camera_prev_pos - self.camera_pos;
        self.camera_prev_pos = self.camera_pos;
        
        return delta
    }

    pub fn get_up(&self) -> Vec3{
        return self.camera_up
    }

    pub fn get_front(&self) -> Vec3{
        return self.camera_front
    }

    pub fn set_front(&mut self, new_front:Vec3){
        self.camera_front = new_front;
    }


    pub fn r#set_x(&mut self, x:f32){
        self.camera_pos[0] = x;
    }

    pub fn r#set_y(&mut self, y:f32){
        self.camera_pos[1] = y;
    }

    pub fn r#set_move(&mut self, pos:Vec3){
        self.camera_pos = pos;
    }

    pub fn r#move(&mut self, direction:Vec3){
        self.camera_pos += direction;
    }

    pub fn r#move_target(&mut self, direction:Vec3){
        self.camera_target += direction;
    }

    pub fn r#change_target(&mut self, new_target:Vec3){
        self.camera_target = new_target;
    }

    pub fn look_at(&self, target: Vec3) -> Mat4{
        let f = (target-self.camera_pos).normalize();
        let mut u = self.camera_up.normalize();
        let s = (f.cross(u)).normalize();
        u = s.cross(f);

        let rotation = Mat4::from_cols(
            s.extend(0.0),
            u.extend(0.0),
            f.extend(0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );    
        let translation = Mat4::from_translation(-self.camera_pos);

        rotation * translation
    }

    pub fn look_at_glm(pos:Vec3, target:Vec3,up:Vec3) -> Mat4{
        let f = (target-pos).normalize();
        let mut u = up.normalize();
        let s = (f.cross(u)).normalize();
        u = s.cross(f);

        let rotation = Mat4::from_cols(
            s.extend(0.0),
            u.extend(0.0),
            f.extend(0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );    
        let translation = Mat4::from_translation(-pos);

        rotation * translation
    }

    pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

}