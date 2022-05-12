use std::collections::HashMap;
use egui::Vec2;
use crate::demo::{Block, BlockPosition};

pub struct BoardState {
    pub(crate) positions: HashMap<String, BlockPosition>,
    pub(crate) blocks: HashMap<String, Block>,
    pub(crate) ids: Vec<String>,
}

impl Default for BoardState {
    fn default() -> Self {
        let positions = HashMap::new();
        let blocks = HashMap::new();
        let ids = Vec::new();
        return Self {
            positions,
            blocks,
            ids,
        }
    }
}

