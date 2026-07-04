

use std::collections::HashMap;

use macroquad::prelude::*;

use crate::core::Block::*;

#[derive(Clone)]
pub struct Renderer {


    pub BlocksToRender : HashMap<(i32,i32,i32),i32>,
    pub BlockList : HashMap<i32,Block>,
    pub material : Material




}

pub const fragment_shader:&str = r#"#version 100
        precision mediump float;
        varying vec2 uv;
        uniform float _Time; // Automatically provided by Macroquad

        void main() {
            gl_FragColor = vec4(uv.x, uv.y, 0.5 + 0.5 * sin(_Time), 1.0);
        }
    "#;

pub const vertex_shader:&str = r#"#version 100
        attribute vec3 position;
        attribute vec2 texcoord;
        uniform mat4 Model;
        uniform mat4 Projection;
        varying vec2 uv;

        void main() {
            gl_Position = Projection * Model * vec4(position, 1.0);
            uv = texcoord;
        }
    "#;

// these shaders are useless for now!



impl Renderer {

    pub fn new(BLOCKS:HashMap<i32,Block>) -> Renderer {


        Renderer { BlocksToRender: HashMap::new(),BlockList : BLOCKS, material : load_material(ShaderSource::Glsl { vertex: vertex_shader, fragment: fragment_shader }, MaterialParams {..Default::default()}).unwrap()
     }

    }

    pub fn render(&self) {

        let mut positions = (self.BlocksToRender).clone().into_keys();
        let mut blocks = (self.BlocksToRender).clone().into_values();

        for pos in positions {

            let mut vec3_pos = Vec3::new(pos.0 as f32, pos.1 as f32, pos.2 as f32);
            
            let block_id = self.BlocksToRender[&pos];
            let block = &self.BlockList[&block_id];
            let _block_textures = block.clone().textures.unwrap();



            // directional textures are not included! Ima try to add them
            let texture = _block_textures.get(&"up".to_string());

            texture.unwrap().set_filter(FilterMode::Nearest);

            //gl_use_material(&self.material);
            draw_cube(vec3_pos, Vec3 { x: 1.0, y: 1.0, z: 1.0 }, texture, block.color);
            //gl_use_default_material();
            


        }




    }

    



}