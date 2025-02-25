use std::collections::{hash_set::Iter, HashSet};

use glam::Vec2;
use winit::{event::{KeyEvent}, keyboard::{self, PhysicalKey}};

pub struct InputHandler{
    movement: Vec2,
    pressed_keys: HashSet<PhysicalKey>, //Maybe make into a map and it has to be processed before it is removed...
}

impl InputHandler{


    pub fn new() -> InputHandler{
        InputHandler{
            movement: Vec2::ZERO,
            pressed_keys: HashSet::new(),
        }
    }

    pub fn get_movement(&self) -> Vec2{
        return self.movement;
    }

    pub fn get_pressed(&self) -> Iter<PhysicalKey>{
        return self.pressed_keys.iter();
    }

    pub fn remove_pressed(&mut self, remove_key: &PhysicalKey){
        self.pressed_keys.remove(remove_key);
    }

    pub fn update_input(&mut self, event: KeyEvent){
        let key = event.physical_key;
        if event.repeat{

        }else if event.state.is_pressed(){
            //Maybe actually divide these into four different if statements..
            //Is probably faster and more clean
            //Key is Y-movement
            if key == keyboard::KeyCode::KeyS{
                self.movement[1] -= 1.0;
            }
            else if key == keyboard::KeyCode::KeyW{
                self.movement[1] += 1.0;
            } // Key is X-movement
            else if key == keyboard::KeyCode::KeyA{
                self.movement[0] -= 1.0;
            }
            else if key == keyboard::KeyCode::KeyD{
                self.movement[0] += 1.0;
            }else //Otherwise put into pressed_keys set
            {
                self.pressed_keys.insert(key);
            }
        }else{
            //Key is Y-movement
            if key == keyboard::KeyCode::KeyS{
                self.movement[1] += 1.0;
            }
            else if key == keyboard::KeyCode::KeyW{
                self.movement[1] -= 1.0;
            } // Key is X-movement
            else if key == keyboard::KeyCode::KeyA{
                self.movement[0] += 1.0;
            }
            else if key == keyboard::KeyCode::KeyD{
                self.movement[0] -= 1.0;
            }else //Otherwise put into pressed_keys set
            {
                self.pressed_keys.remove(&key);
            }
        }
    }
}