use std::fmt::{Display, Formatter};
use egui::{Vec2, Widget};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug, Clone)]
pub struct BlockPosition {
    pub(crate) id: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) size: Vec2
}

#[derive(Debug, Copy, Clone)]
pub enum BlockType {
    Button,
    Label
}

impl Display for BlockType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromSql for BlockType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let v = value.as_str().ok().unwrap();
        return Ok(BlockType::Label);
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub(crate) id: String,
    pub(crate) block_type: BlockType,
    pub(crate) block_data: String,
}