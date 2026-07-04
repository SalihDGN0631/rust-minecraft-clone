use macroquad::prelude::*;

#[derive(Clone)]
pub struct FirstPersonController {

    pub pitch : f32,
    pub yaw : f32,
    pub mouse_grab : bool,
    pub sensitivity : f32,
    pub move_speed : f32




}

impl FirstPersonController {


    pub fn new() -> FirstPersonController {
        FirstPersonController { 
            pitch : 0.0,
            yaw : 0.0,
            mouse_grab : false,
            sensitivity : 25.0,
            move_speed : 15.0
         }
    }

    

    pub fn enable(&mut self) {
        self.mouse_grab = true;
        
        
        show_mouse(false);
        set_cursor_grab(true);
    }

    pub fn disable(&mut self) {

        self.mouse_grab = false;
        show_mouse(true);
        set_cursor_grab(false);

    }


    pub fn step(&mut self,cam_pos:Vec3,delta_time:f32) -> (Vec3,Vec3) {

        

        let delta = mouse_delta_position();
        self.yaw -= delta.x * self.sensitivity;
        self.pitch += delta.y * self.sensitivity;

        self.pitch = self.pitch.clamp(-1.5, 1.5);
        

        let front = vec3(
         self.yaw.cos() * self.pitch.cos(),  
         self.pitch.sin(),                   
         self.yaw.sin() * self.pitch.cos(),  
        ).normalize();


        if self.mouse_grab == false {
            return (front,cam_pos);
        }

        let mut position = cam_pos;

        let mut direction = Vec3::ZERO;

        if is_key_down(KeyCode::W) { direction.z += 1.0; } 
        if is_key_down(KeyCode::S) { direction.z -= 1.0; } 
        if is_key_down(KeyCode::A) { direction.x -= 1.0; } 
        if is_key_down(KeyCode::D) { direction.x += 1.0; } 
        if is_key_down(KeyCode::LeftShift) {direction.y -= 1.0;}
        if is_key_down(KeyCode::Space) {direction.y += 1.0;}

        if direction != Vec3::ZERO {
            direction = direction.normalize();

            let right = vec3(-self.yaw.sin(), 0.0, self.yaw.cos()).normalize();
            let forward = vec3(self.yaw.cos(), 0.0, self.yaw.sin()).normalize();

            let mspeed = self.move_speed * delta_time;

            let new_pos = (forward * direction.z) + (right * direction.x) + (vec3(0.0, 1.0, 0.0) * direction.y);

            position += new_pos * mspeed;
            
        }

        




        ((front + cam_pos),position)
    }

    

}