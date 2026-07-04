


use std::fmt::Error;

use macroquad::{miniquad::conf::Platform, prelude::*};

use crate::core::{Registry::Registry, Renderer::{Renderer}};

pub fn _window_conf() -> Conf {
    Conf {

        platform : Platform {
            
            ..Default::default()
        },

        window_title : "Minecraft 1.0.0".to_string(),
        window_height : 1080,
        window_width : 1920,
        fullscreen : true,
        
        ..Default::default()

    }
}

const DEFAULT_SKYBOX_COLOR: Color = Color::from_rgba(130, 177, 195, 1);



pub struct Game {

    pub cam3d : Camera3D,
    pub delta_time : f32,
    pub renderer : Renderer,
    pub registry : Registry


}




impl Game {

    pub fn new(_reg:Registry) -> Game {

        Game {
            cam3d : Camera3D{..Default::default()},
            delta_time : 0.0 as f32,
            registry : _reg.clone(),
            renderer : Renderer::new(_reg.Blocks),
        }

    }

    pub fn setup(&mut self) {

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


        set_camera(&self.cam3d);

        //3D Drawing
        self.renderer.render();


        set_default_camera();

        // UI Drawing

        next_frame().await;


    }

}