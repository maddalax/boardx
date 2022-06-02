use egui::{Pos2, Vec2};
use egui::Key::N;
use rusqlite::{Connection, MappedRows, params, Row};
use std::sync::{Arc, RwLock};

use crate::demo::BlockType;
use crate::state::BoardState;

pub struct Persistor {}

#[derive(Debug)]
pub struct SavedBlock {
    pub(crate) size: Vec2,
    pub(crate) position: Pos2,
    pub(crate) id: String,
    pub(crate) block_type: BlockType,
    pub(crate) block_data: String,
}

impl Default for Persistor {
    fn default() -> Self {
        return Self {};
    }
}

impl Persistor {
    thread_local! {
        static CONNECTION: RwLock<Arc<Connection>> = RwLock::new(Arc::new(Connection::open("./boardx.db").unwrap()));
    }

    pub fn setup(&mut self) {
        let connection = Persistor::CONNECTION.with(|c| c.read().unwrap().clone());
        connection.execute("CREATE TABLE IF NOT EXISTS blocks (id TEXT, type INTEGER, data TEXT, x REAL, y REAL);", params![])
            .unwrap();
    }

    pub fn on_size_change(&mut self, id: &String, size: Vec2) {
        let connection = Persistor::CONNECTION.with(|c| c.read().unwrap().clone());
        connection
            .execute("UPDATE blocks SET width = ?, height = ? WHERE id = ?", [size.x.to_string(),
                size.y.to_string(), id.to_string()]).unwrap();
    }

    pub fn on_add(&mut self, block: SavedBlock) {
        let connection = Persistor::CONNECTION.with(|c| c.read().unwrap().clone());
        connection
            .execute("INSERT INTO blocks VALUES(?, ?, ?, ?, ?, ?, ?)", [
                block.id,
                block.block_type.to_string(),
                block.block_data,
                block.position.x.to_string(),
                block.position.y.to_string(),
                block.size.x.to_string(),
                block.size.y.to_string()]).unwrap();
    }

    pub fn on_move(&mut self, id: &String, x: f32, y: f32) {
        println!("block moved: {}, {}, {}", id, x, y);
        let connection = Persistor::CONNECTION.with(|c| c.read().unwrap().clone());
        connection
            .execute("UPDATE blocks SET x = ?, y = ? WHERE id = ?", [
                x.to_string(), y.to_string(), id.to_string()]).unwrap();
    }

    pub fn on_data_change(&mut self, id: &String, data: String) {
        let connection = Persistor::CONNECTION.with(|c| c.read().unwrap().clone());
        connection
            .execute("UPDATE blocks SET data = ? WHERE id = ?", [
                data, id.to_string()]).unwrap();
    }

    pub fn load(self, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Vec<SavedBlock> {
        let connection = Persistor::CONNECTION.with(|c| c.read().unwrap().clone());

        let query = format!("SELECT id, type, data, x, y, height, width FROM blocks WHERE {} < x + width AND {} > x AND {} < y + height AND {} > y",
                            x_min, x_max, y_min, y_max);

        println!("{}", query);

        let mut stmt = connection.prepare(&query).unwrap();

        let block_iter = stmt.query_map([], |row| {
            Ok(SavedBlock {
                size: Vec2::new(row.get(5)?, row.get(6)?),
                position: Pos2::new(row.get(3)?, row.get(4)?),
                id: row.get(0)?,
                block_type: row.get(1)?,
                block_data: row.get(2)?,
            })
        }).unwrap();


        let mut blocks: Vec<SavedBlock> = Vec::new();

        for block in block_iter {
            blocks.push(block.unwrap());
        }

        println!("total blocks: {}", blocks.len());

        return blocks;
    }
}