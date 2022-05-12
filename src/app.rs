use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::{fs, thread};
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use uuid::Uuid;

use eframe::emath::{Align2, Vec2};
use eframe::epaint::{Color32, Rgba};

use egui_extras::RetainedImage;

use egui::{Id, LayerId, Order, TextStyle, Visuals, Widget, DroppedFile, ColorImage, Rect, Pos2, FontId, Layout, Sense, Direction, PointerState, Key};
use rand::{random, Rng, RngCore};
use crate::demo::{Block, BlockPosition, BlockType};
use crate::state::BoardState;
use crate::view::ViewState;

pub struct App {
    // Example stuff:

    // image: Option<Result<RetainedImage, String>>,

    block_state: BoardState,

    view_state: ViewState,

    dragging_widget: String,

    selected_widget: String,

    rendered_blocks: i32,

    total_blocks: i32,

    pixels_per_point: f32,

    last_pixels_per_point: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            dragging_widget: String::from(""),
            selected_widget: String::from(""),
            block_state: BoardState::default(),
            view_state: ViewState::default(),
            rendered_blocks: 0,
            total_blocks: 0,
            pixels_per_point: 0.0,
            last_pixels_per_point: 0.0,
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
            let x = interact_point.unwrap().x + self.view_state.screen.x;
            let y = interact_point.unwrap().y + self.view_state.screen.y;
            return Pos2::new(x, y)
        }
        return Pos2::default()
    }

    pub fn add_label(&mut self, x: f32, y: f32) {
        let id = Uuid::new_v4().to_string();
        self.block_state.ids.push(id.clone());
        self.block_state.blocks.insert(id.clone(), Block{
            id: Uuid::new_v4().to_string(),
            block_type: BlockType::Label,
            block_data: String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."),
        });
        self.block_state.positions.insert(id.clone(), BlockPosition {
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

        if self.pixels_per_point <= 0.00 {
            self.pixels_per_point = ctx.pixels_per_point();
        }

        if self.pixels_per_point > 5.00 {
            self.pixels_per_point = 5.00;
        }

        if self.last_pixels_per_point != self.pixels_per_point {
            self.last_pixels_per_point = self.pixels_per_point;
        }

        if ctx.input().key_down(Key::ArrowDown) {
            self.pixels_per_point -= 0.05;
        }

        if ctx.input().key_down(Key::ArrowUp) {
            self.pixels_per_point += 0.05;

        }

        // let mut scroll_delta =  ctx.input().scroll_delta.y;
        //
        // if scroll_delta > 500.00 {
        //     scroll_delta = 500.00
        // }
        //
        // if scroll_delta < -500.00 {
        //     scroll_delta = -500.00
        // }
        //
        // let normalized = (scroll_delta * 10.00) / 500.00;
        //
        // if scroll_delta != 0.00 {
        //     println!("Delta Y: {}, {}", scroll_delta, normalized);
        // }

        // self.pixels_per_point += normalized;

        // if scroll_delta > 200.00 {
        //     self.pixels_per_point += 1.00;
        // }
        //
        // if scroll_delta < -200.00 {
        //     self.pixels_per_point -= 1.00;
        // }

        let screen_size = ctx.input().screen_rect().size();
        self.view_state.viewport = screen_size + self.view_state.screen;

        let pointer = ctx.input().pointer.clone();

        if !pointer.any_down() {
            self.dragging_widget = String::from("")
        }

        if pointer.any_down() && pointer.interact_pos().is_some() && self.dragging_widget == "" {
            let interact_point = self.get_interact_point(&pointer);
            let x = interact_point.x;
            let y = interact_point.y;

            for id in &self.block_state.ids {
                let block_position = self.block_state.positions.get(id).unwrap();
                let widget_size = block_position.size;
                if x >= block_position.x && x <= block_position.x + widget_size.x && y >= block_position.y && y <= block_position.y + widget_size.y {
                    self.dragging_widget = block_position.id.clone();
                    self.selected_widget = block_position.id.clone();
                }
            }

        }

        if pointer.any_down() && pointer.is_moving() {
            if self.dragging_widget != "" {
                let position = self.block_state.positions.get_mut(&self.dragging_widget).unwrap();
                position.x += pointer.delta().x;
                position.y += pointer.delta().y
            } else {
                self.view_state.screen.x -= pointer.delta().x;
                self.view_state.screen.y -= pointer.delta().y;
            }
        }

        if pointer.any_click() && self.dragging_widget == "" {
            let interact_point = self.get_interact_point(&pointer);
            self.add_label(interact_point.x, interact_point.y);
        }

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

            ui.add(egui::Slider::new(&mut self.pixels_per_point, 0.0..=10.00).text("Pixels Per Point"));

            if self.selected_widget != "" {

                let block_position = self.block_state.positions.get_mut(&self.selected_widget).unwrap();

                ui.add(egui::Slider::new(&mut block_position.size.x, 0.0..=1000.00).text("Selected Widget Width"));
                ui.add(egui::Slider::new(&mut block_position.size.y, 0.0..=1000.00).text("Selected Widget Height"));
            }

            ui.label(format!("Screen Offset: {}, {}", self.view_state.screen.x, self.view_state.screen.y));
            ui.label(format!("Screen Size: {}, {}", screen_size.x, screen_size.y));

            let clip_rect = ui.clip_rect();

            ui.label(format!("Area: {} - {}, {} - {}", self.view_state.screen.x, self.view_state.viewport.x, self.view_state.screen.y, self.view_state.viewport.y));

            ui.label(format!("Dragging Widget: {}", self.dragging_widget));

            ui.label(format!("Clip Rect: {}, {}", clip_rect.size().x, clip_rect.size().y));

            ui.label(format!("Rendered Blocks: {}", self.rendered_blocks));
            ui.label(format!("Total Blocks: {}", self.total_blocks));


        });

        egui::CentralPanel::default().show(ctx, |ui| {

            self.rendered_blocks = 0;
            self.total_blocks = 0;
            for id in &self.block_state.ids {
                self.total_blocks += 1;
                let mut block_position = self.block_state.positions.get_mut(id).unwrap();
                if !self.view_state.in_viewport(block_position.x, block_position.y) {
                    continue;
                }

                self.rendered_blocks += 1;

                let position = Pos2::new(block_position.x - self.view_state.screen.x, block_position.y - self.view_state.screen.y);

                let block = self.block_state.blocks.get(&block_position.id).unwrap();
                let data = &block.block_data;
                match block.block_type {
                    BlockType::Button => {
                        // ui.put(widget_rect, egui::Button::new(data));
                    },
                    BlockType::Label => {
                        let old_clip_rect = ui.clip_rect();
                        ui.set_clip_rect(Rect::NOTHING);

                        let r = egui::Label::new(data).wrap(true).ui(ui);

                        let mut rect = r.rect;

                        // Clamp to max 300 width by default
                        if block_position.size.x == 0.00 {
                            if rect.width() > 300.00 {
                                rect.set_width(300.00);
                            }
                        } else {
                            rect.set_width(block_position.size.x);
                        }

                        let size = rect.size();

                        let widget_rect = Rect::from_min_size(position, size);

                        block_position.size = size;

                        ui.set_clip_rect(old_clip_rect);

                        let r2 = ui.put(widget_rect, egui::Label::new(data).wrap(true));
                        block_position.size = r2.rect.size();
                    }
                }
            }
        });
    }
}