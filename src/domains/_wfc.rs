use bevy::utils::HashSet;

use crate::prelude::*;

use arr_macro::arr;

pub const TILE_COUNT: usize = 12 * 16;

pub struct WFCBuilder{
    pub arena: Vec<WFCNode> ,
}
impl Default for WFCBuilder {
    fn default() -> Self{
        let arena = vec![WFCNode::default(); TILE_COUNT];
        Self { arena }
    }
}

impl WFCBuilder {
    pub fn add_sample(
        &mut self,
        index: usize,
        up: Option<usize>,
        down: Option<usize>,
        left: Option<usize>,
        right: Option<usize>
    ){
        if let Some(i) = up{
            self.arena[index].up.insert(i);
        }
        if let Some(i) = down{
            self.arena[index].down.insert(i);
        }
        if let Some(i) = left{
            self.arena[index].left.insert(i);
        }
        if let Some(i) = right{
            self.arena[index].right.insert(i);
        }
    }

    pub fn get_left(
        & self,
        index: usize
    ) -> &HashSet<usize>{
        &self.arena[index].left
    }

    pub fn get_right(
        &self,
        index: usize
    ) -> &HashSet<usize>{
        &self.arena[index].right
    }

    pub fn get_up(
        &self,
        index: usize
    ) -> &HashSet<usize>{
        &self.arena[index].up
    }

    pub fn get_down(
        &self,
        index: usize
    ) -> &HashSet<usize>{
        &self.arena[index].down
    }

    pub fn ingest(&mut self, input: &[usize], x_size: usize, y_size: usize){
        for y in 0..y_size{
            for x in 0..x_size{
                let up = if y == 0{
                    None
                } else {
                    Some(index(x, y - 1, x_size))
                };

                let down = if y == (y_size - 1){
                    None
                } else {
                    Some(index(x, y + 1, x_size))
                };

                let left = if x == 0 {
                    None
                } else {
                    Some(index(x - 1, y, x_size))
                };

                let right = if x == (x_size -1){
                    None
                }else{
                    Some(index(x +1, y, x_size))
                };

                self.add_sample(index(x, y, x_size), up, down, left, right);
            }
        }
    }
}

fn index(x: usize, y: usize, max_x: usize) -> usize{
    y * max_x + x
}

#[derive(Default, Clone)]
pub struct WFCNode{
    pub left: HashSet<usize>,
    pub right: HashSet<usize>,
    pub up: HashSet<usize>,
    pub down: HashSet<usize>,
}

pub struct WFCOutput{
    arena: Vec<WFCOutputNode> 
}
impl Default for WFCOutput {
    fn default() -> Self{
        let arena = vec![WFCOutputNode::default(); TILE_COUNT];
        Self{
            arena
        }
    }
}

impl WFCOutput {
    pub fn from_builder(builder: &WFCBuilder) -> Self{
        let mut output = Self::default();
        for i in 0..TILE_COUNT{
            let left = builder.get_left(i).iter().map(|x|{*x}).collect();
            let right = builder.get_right(i).iter().map(|x|{*x}).collect();
            let up = builder.get_up(i).iter().map(|x|{*x}).collect();
            let down = builder.get_down(i).iter().map(|x|{*x}).collect();

            output.arena[i] = WFCOutputNode {
                left,
                right,
                up,
                down
            };
        }
        output
    }
}

#[derive(Default, Clone)]
pub struct WFCOutputNode{
    pub left: Vec<usize>,
    pub right: Vec<usize>,
    pub up: Vec<usize>,
    pub down: Vec<usize>
}
