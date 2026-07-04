mod core;

use core::game::*;

use macroquad::{color::{BLACK, BLUE, Color, GREEN, LIME, RED, WHITE, YELLOW}, input::KeyCode::D, material::{Material, MaterialParams, load_material}, miniquad::{ShaderSource, gl::glCreateShader}, texture::Texture2D};

use crate::core::{Block::Block, Registry::Registry};



#[macroquad::main(_window_conf)]
async fn main() {

    let mut _reg = Registry::new();

    _reg.register_block(Block::newBlock(
        
        Texture2D::from_file_with_format(include_bytes!("../assets/stone.png"), None)
        , core::Block::BlockType::STONE, WHITE)
    ); // block code is 0
    _reg.register_block(Block::newBlock(
        Texture2D::from_file_with_format(include_bytes!("../assets/dirt.png"), None), 
        core::Block::BlockType::DIRT, 
        WHITE)); // block code is 1
    _reg.register_block(Block::newBlock(
        Texture2D::from_file_with_format(include_bytes!("../assets/oak_leaves.png"), None)
        , core::Block::BlockType::DIRT
        , LIME // 2
    ));
    _reg.register_block(Block::newBlock(
        Texture2D::from_file_with_format(include_bytes!("../assets/oak_leaves.png"), None),
         core::Block::BlockType::DIRT, 
         RED)); // 3
    _reg.register_block(Block::newBlock(
        Texture2D::from_file_with_format(include_bytes!("../assets/gold_block.png"), None)
        , core::Block::BlockType::METAL
        , WHITE
    ));
    _reg.register_block(Block::newBlock(
        Texture2D::from_file_with_format(include_bytes!("../assets/diamond_block.png"), None)
        , core::Block::BlockType::METAL
        , WHITE
    ));

    

    


    let mut g = Game::new(_reg);

    g.setup();

    loop {

        g.step().await;



    }


    
}