use core::f32::consts::PI;
use nalgebra_glm as glm;

pub struct Player {
    pub position: glm::Vec2,
    pub direction: glm::Vec2,
    pub view_angle: f32,
    pub fov: f32
}

impl Player {
    pub fn new(x:f32,y:f32,angle:f32,fov:f32) -> Self {
        let direction = glm::vec2(angle.cos(),angle.sin());
        Player {
            position: glm::vec2(x,y),
            direction,
            view_angle: angle,
            fov:PI/3.0,
        }
    }

    pub fn update_direction(&mut self) {
        self.direction = glm::vec2(self.view_angle.cos(),self.view_angle.sin());
    }

    pub fn move_forward(&mut self, distance:f32) {
        self.position += self.direction * distance;
    }

    pub fn rotate(&mut self, angle_change:f32) {
        self.view_angle += angle_change;
        self.update_direction();
    }
}
