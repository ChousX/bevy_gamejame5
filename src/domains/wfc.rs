use bevy::utils::HashSet;

use crate::prelude::*;
use std::collections::hash_set::Iter as SetIter;

pub const TILE_COUNT: usize = 12 * 16;

pub struct WFCBuilder{
    pub arena: [WFCNode; TILE_COUNT],
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

#[derive(Default)]
pub struct WFCNode{
    pub left: HashSet<usize>,
    pub right: HashSet<usize>,
    pub up: HashSet<usize>,
    pub down: HashSet<usize>,
}

