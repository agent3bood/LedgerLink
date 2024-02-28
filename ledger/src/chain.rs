mod memory;

use crate::block::Block;

trait Chain {
    fn new() -> Self;
    fn verify(&self) -> bool;
    fn get_depth(&self) -> u64;
    fn add_block(&mut self, block: Block) -> bool;
    fn get_block(&self, index: u64) -> Option<&Block>;
    fn get_last_block(&self) -> Option<&Block>;
    fn get_blocks(&self) -> &Vec<Block>;
}
