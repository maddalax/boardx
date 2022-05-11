use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

use eframe::emath::{Align2, Vec2};
use eframe::epaint::{Color32, Rgba};

use egui_extras::RetainedImage;

use egui::{Id, LayerId, Order, TextStyle, Visuals, Widget, DroppedFile, ColorImage, Rect, Pos2, FontId, Layout, Sense, Direction, PointerState};
use rand::Rng;
use crate::demo::{Block, BlockPosition, BlockType};

pub struct App {
    // Example stuff:
    label: String,

    value: f32,

    image: Option<Result<RetainedImage, String>>,

    screen: Vec2,

    dragging_widget: String,

    selected_widget: String,

    positions: HashMap<String, BlockPosition>,

    blocks: HashMap<String, Block>,

    ids: Vec<String>
}

impl Default for App {
    fn default() -> Self {

        let mut positions = HashMap::new();
        let mut blocks = HashMap::new();
        let mut ids = Vec::new();

        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            image: None,
            screen: Vec2::new(0.00, 0.00),
            dragging_widget: String::from(""),
            selected_widget: String::from(""),
            positions,
            blocks,
            ids,
        }
    }
}


impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_visuals(Visuals::dark());

        Default::default()
    }

    pub fn get_interact_point(&self, state: &PointerState) -> Pos2 {
        let interact_point = state.interact_pos();
        if interact_point.is_some() {
            let x = interact_point.unwrap().x + self.screen.x;
            let y = interact_point.unwrap().y + self.screen.y;
            return Pos2::new(x, y)
        }
        return Pos2::default()
    }

    pub fn add_label(&mut self, x: f32, y: f32) {
        let id = Uuid::new_v4().to_string();
        self.ids.push(id.clone());
        self.blocks.insert(id.clone(), Block{
            id: Uuid::new_v4().to_string(),
            block_type: BlockType::Label,
            block_data: String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."),
        });
        self.positions.insert(id.clone(), BlockPosition {
            id: id.clone(),
            x,
            y,
            size: Vec2::new(0.00, 0.00),
        });
    }
}

impl eframe::App for App {

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        ctx.set_debug_on_hover(true);

        let screen_size = ctx.input().screen_rect().size();

        let pointer = ctx.input().pointer.clone();

        if !pointer.any_down() {
            self.dragging_widget = String::from("")
        }

        if pointer.any_down() && pointer.interact_pos().is_some() && self.dragging_widget == "" {
            let interact_point = self.get_interact_point(&pointer);
            let x = interact_point.x;
            let y = interact_point.y;

            for id in &self.ids {
                let block_position = self.positions.get(id).unwrap();
                let widget_size = block_position.size;
                if x >= block_position.x && x <= block_position.x + widget_size.x && y >= block_position.y && y <= block_position.y + widget_size.y {
                    self.dragging_widget = block_position.id.clone();
                    self.selected_widget = block_position.id.clone();
                }
            }

        }

        if pointer.any_down() && pointer.is_moving() {
            if self.dragging_widget != "" {
                let position = self.positions.get_mut(&self.dragging_widget).unwrap();
                position.x += pointer.delta().x;
                position.y += pointer.delta().y
            } else {
                self.screen.x -= pointer.delta().x;
                self.screen.y -= pointer.delta().y;
            }
        }

        if pointer.any_click() && self.dragging_widget == "" {
            let interact_point = self.get_interact_point(&pointer);
            self.add_label(interact_point.x, interact_point.y);
        }

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {

            ui.heading("Side Panel");


            if self.selected_widget != "" {

                let block_position = self.positions.get_mut(&self.selected_widget).unwrap();

                ui.add(egui::Slider::new(&mut block_position.size.x, 0.0..=1000.00).text("Selected Widget Width"));
                ui.add(egui::Slider::new(&mut block_position.size.y, 0.0..=1000.00).text("Selected Widget Height"));
            }

            ui.label(format!("Screen Offset: {}, {}", self.screen.x, self.screen.y));
            ui.label(format!("Screen Size: {}, {}", screen_size.x, screen_size.y));

            let area = screen_size + self.screen;

            let clip_rect = ui.clip_rect();

            ui.label(format!("Area: {}, {}", area.x, area.y));

            ui.label(format!("Dragging Widget: {}", self.dragging_widget));

            ui.label(format!("Clip Rect: {}, {}", clip_rect.size().x, clip_rect.size().y))

        });

        egui::CentralPanel::default().show(ctx, |ui| {

            for id in &self.ids {
                let mut block_position = self.positions.get_mut(id).unwrap();
                let position = Pos2::new(block_position.x - self.screen.x, block_position.y - self.screen.y);
                let block = self.blocks.get(&block_position.id).unwrap();
                let data = &block.block_data;
                match block.block_type {
                    BlockType::Button => {
                        // ui.put(widget_rect, egui::Button::new(data));
                    },
                    BlockType::Label => {
                        let old_clip_rect = ui.clip_rect();
                        // let old_cursor = ui.cursor();
                        ui.set_clip_rect(Rect::NOTHING);

                        let r = egui::Label::new(data).wrap(true).ui(ui);

                        let mut rect = r.rect;

                        // Clamp to max 300 width by default
                        if block_position.size.x == 0.00 {
                            if rect.width() > 300.00 {
                                block_position.size.x = 300.00;
                            } else {
                                block_position.size.x = rect.width();
                            }
                        }

                        if block_position.size.x <= rect.width() {
                            rect.set_width(block_position.size.x);
                        }

                        let size = rect.size();

                        let widget_rect = Rect::from_min_size(position, size);

                        block_position.size = size;

                        ui.set_clip_rect(old_clip_rect);
                        ui.set_max_width(300.00);

                        let r2 = ui.put(widget_rect, egui::Label::new(data).wrap(true));
                        block_position.size = r2.rect.size();
                    }
                }
            }
        });
    }
}