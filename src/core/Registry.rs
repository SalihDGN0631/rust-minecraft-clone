use std::collections::HashMap;

use crate::core::Block::Block;

#[derive(Clone)]
pub struct Registry {

    pub Blocks : HashMap<i32,Block>,
    


    
}

impl Registry {


    pub fn new() -> Registry {

        Registry{

            Blocks : HashMap::new()

        }


    }

    pub fn register_block(&mut self,b:Block) {
        &self.Blocks.insert(self.Blocks.len() as i32, b);


    }

    pub fn getBlocks(&self) -> HashMap<i32,Block> {

        
        return self.Blocks.clone();
    }

}