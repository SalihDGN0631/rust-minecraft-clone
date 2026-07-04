


use std::fmt::Error;

use macroquad::{miniquad::conf::Platform, prelude::*};

use crate::core::{FirstPersonController::FirstPersonController, Registry::Registry, Renderer::Renderer};

pub fn _window_conf() -> Conf {
    Conf {

        platform : Platform {
            swap_interval : Some(1),
            ..Default::default()
        },

        window_title : "Minecraft 1.0.0".to_string(),
        window_height : 1080,
        window_width : 1920,
        fullscreen : true,
        
        ..Default::default()

    }
}

const DEFAULT_SKYBOX_COLOR: Color = Color::from_rgba(135, 206, 250, 1);



pub struct Game {

    pub cam3d : Camera3D,
    pub delta_time : f32,
    pub renderer : Renderer,
    pub registry : Registry,
    pub fpc : FirstPersonController


}


const DEBUG:bool = true;

impl Game {

    pub fn new(_reg:Registry) -> Game {

        Game {
            cam3d : Camera3D{..Default::default()},
            delta_time : 0.0 as f32,
            registry : _reg.clone(),
            renderer : Renderer::new(_reg.Blocks),
            fpc : FirstPersonController { pitch: 0.0, yaw: 0.0, mouse_grab: false, sensitivity: 0.3, move_speed: 5.0 }
        }

    }

    pub fn setup(&mut self) {
        self.fpc.enable();
        self.cam3d.up = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        self.cam3d.position = vec3(5.0, 10.0, 5.0);
        //self.cam3d.fovy = 90.0;

        self.renderer.BlocksToRender.insert((1,1,1), 0); // Stone
        self.renderer.BlocksToRender.insert((1,1,2),1); // Dirt
        self.renderer.BlocksToRender.insert((1,1,3),5); // Diamond Block
        self.renderer.BlocksToRender.insert((2,1,3),2); // Leaf (Green)
        self.renderer.BlocksToRender.insert((2,1,2),3); // Leaf (Blue)
        self.renderer.BlocksToRender.insert((2,1,1),4); // Gold Block
    }
    

    pub async fn step(&mut self) {

        self.delta_time = get_frame_time();

        clear_background(DEFAULT_SKYBOX_COLOR);


        
        let camTarget = self.fpc.step(self.cam3d.position,self.delta_time);
        //self.cam3d.position = self.fpc.step_move(self.cam3d.position);
        self.cam3d.target = camTarget.0;
        self.cam3d.position = camTarget.1;
        


        set_camera(&self.cam3d);

        //3D Drawing
        self.renderer.render();


        set_default_camera();

        if DEBUG == true {

            draw_text(format!("FPS : {}",get_fps()), 20.0, 25.0, 20.0, BLACK);
            draw_text(format!("Coordinates : X {},Y {},Z {}",self.cam3d.position.x,self.cam3d.position.y,self.cam3d.position.z), 150.0, 25.0, 20.0, BLACK);
            draw_text(format!("Pitch : {}, Yaw : {}",self.fpc.pitch,self.fpc.yaw), 600.0, 25.0, 20.0, BLACK);
            
        }

        // UI Drawing

        next_frame().await;


    }

}