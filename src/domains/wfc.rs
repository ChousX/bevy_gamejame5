use bevy::{ecs::component::StorageType, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use crate::prelude::*;
use rand::prelude::*;
use std::cmp::Ordering;

pub struct WFCPlugin;
impl Plugin for WFCPlugin{
    fn name(&self) -> &str {
        "Wave Function Collaps Plugin"
    }

    fn build(&self, app: &mut App) {
        app
            .init_resource::<WFCRules>()
    ;}
}

#[derive(Resource, Default)]
pub struct WFCRules{
    tiles: Vec<WFCRuleNode>,
}

struct WFCRuleNode {
    left: HashSet<u32>,
    right: HashSet<u32>,
    up: HashSet<u32>,
    down: HashSet<u32>,
}

impl WFCRules {
    pub fn add(&mut self, index: usize, possibility: u32, direction: Direction) {
        match direction{
            Direction::Up => &mut self.tiles[index].up,
            Direction::Down => &mut self.tiles[index].down,
            Direction::Left => &mut self.tiles[index].left,
            Direction::Right => &mut self.tiles[index].right,
        }.insert(possibility);
    }

    pub fn build_node(&self) -> WFCNode{
        let len = self.tiles.len();
        WFCNode::new(len as u32)
    }
}

#[derive(Component)]
pub struct WFCNode{
    possibilities: HashSet<u32>
}

impl WFCNode {
    pub fn new(count: u32) -> Self{
        let acum: HashSet<u32> = (0..count).collect();
        Self{
            possibilities: acum,
        }
    }

    pub fn update(&mut self, other: &HashSet<u32>) {
        let temp = self.possibilities.intersection(other).map(|x| {*x});
        self.possibilities = temp.collect();
    }

    pub fn collaps(&mut self) -> u32 {
        let acum: Vec<_> = self.possibilities.drain().collect();
        let pick = acum[thread_rng().gen_range(0..acum.len())].clone();
        self.possibilities.insert(pick.clone());
        pick
    }

    pub fn get_entropy(&self) -> usize{
        self.possibilities.len()
    }
}

impl PartialOrd for WFCNode {
    fn partial_cmp(&self, other: &WFCNode) -> Option<Ordering> { 
        Some(self.get_entropy().cmp(&other.get_entropy()))
    }
}

impl Ord for WFCNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_entropy().cmp(&other.get_entropy()) 
    }
}

impl PartialEq for WFCNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_entropy() == self.get_entropy()
    }
}

impl Eq for WFCNode {}


#[derive(Copy, Clone)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn wave_function_collaps(
    tilemap: Query<(&TileStorage, &TilemapSize)>,
    mut wfc_nodes: Query<&mut WFCNode>,
    rules: Res<WFCRules>
) {
    // 0 make q
    let mut q = BinaryHeap::new();
    // fill q 
    let (storage, size) = tilemap.single();
    for y in 0..size.y{
        for x in 0..size.x{
            if let Some(entity) = storage.get(&TilePos {x, y}) {
                let wfc_node = wfc_nodes.get(entity);
                q.push(wfc_node);
            } 
        }
    }
    // iter through q I hope this is getting the lowest
    while let Some(node) = q.pop() {
        //
    }

}

