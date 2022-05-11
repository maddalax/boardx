use egui::{Vec2, Widget};

pub mod text_edit;

pub struct BlockPosition {
    pub(crate) id: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) size: Vec2
}

pub enum BlockType {
    Button,
    Label
}

pub struct Block {
    pub(crate) id: String,
    pub(crate) block_type: BlockType,
    pub(crate) block_data: String,
}