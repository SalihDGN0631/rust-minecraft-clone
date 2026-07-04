

use std::collections::HashMap;

use macroquad::prelude::*;


//just definition of the block 

#[derive(Clone)]
pub enum BlockType {

    METAL,
    DIRT,
    STONE
}

#[derive(Clone)]
pub struct Block  {

    pub textures : Option<HashMap<String,Texture2D>>,
    pub block_type : Option<BlockType>,
    pub color : Color,




}




impl Block {

    pub fn new(textures:Option<HashMap<String,Texture2D>>,_type:BlockType,_color : Color) -> Block {

        Block {

            textures : textures,
            block_type : Some(_type),
            color : _color

        }


    }

    pub fn newBlock(texture:Texture2D,_type:BlockType,_color:Color) -> Block {
        let mut textures:HashMap<String, Texture2D> = HashMap::new();

        textures.insert("up".to_string(), texture.clone());
        textures.insert("down".to_string(),texture.clone());
        textures.insert("right".to_string(), texture.clone());
        textures.insert("left".to_string(), texture.clone());
        textures.insert("front".to_string(), texture.clone());
        textures.insert("back".to_string(), texture.clone());


        Block {
            textures : Some(textures),
            block_type : Some(_type),
            color : _color


        }



    }

    



}

impl Default for Block {


    fn default() -> Self {
        Block {
            textures : None,
            block_type : Some(BlockType::METAL),
            color : WHITE

        }
    }
}