use std::{fs, thread};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::process::id;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use eframe::emath::{Align2, Vec2};
use eframe::epaint::{Color32, Rgba};
use egui::{ColorImage, Direction, DroppedFile, epaint, FontId, Id, Key, LayerId, Layout, MultiTouchInfo, Order, PointerState, Pos2, pos2, Rect, Sense, Shape, Style, TextStyle, Visuals, Widget};
use egui::Key::{N, S};
use rand::{random, Rng, RngCore};
use uuid::Uuid;

use crate::demo::{Block, BlockPosition, BlockType};
use crate::persistor::{Persistor, SavedBlock};
use crate::state::BoardState;
use crate::view::ViewState;

pub struct App {
    board_state: BoardState,

    view_state: ViewState,

    dragging_widget: String,

    selected_widget: String,

    hovered_widget: String,

    rendered_blocks: i32,

    total_blocks: i32,

    pixels_per_point: f32,

    last_pixels_per_point: f32,

    persist: Persistor,

    view_state_sender: Option<Sender<ViewState>>,

    board_state_receiver: Option<Receiver<BoardState>>,

    initialized: bool,

    last_viewport_change: Instant,

    debug_mode: bool,

    last_click: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            dragging_widget: String::from(""),
            selected_widget: String::from(""),
            hovered_widget: String::from(""),
            board_state: BoardState::default(),
            view_state: ViewState::default(),
            persist: Persistor::default(),
            rendered_blocks: 0,
            total_blocks: 0,
            pixels_per_point: 0.0,
            last_pixels_per_point: 0.0,
            view_state_sender: None,
            board_state_receiver: None,
            initialized: false,
            last_viewport_change: Instant::now(),
            debug_mode: true,
            last_click: Instant::now(),
        }
    }
}

const BUFFER: f32 = 300.00;

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // cc.egui_ctx.set_visuals(Visuals::dark());

        let (view_state_sender, view_state_reciever): (Sender<ViewState>, Receiver<ViewState>) = channel();

        let (board_state_sender, board_state_receiver): (Sender<BoardState>, Receiver<BoardState>) = channel();

        let mut instance = App::default();
        instance.persist.setup();


        thread::spawn(move || {
            loop {
                let persist = Persistor::default();
                let buffer = 300.00;
                match view_state_reciever.try_recv() {
                    Ok(view_state) => {
                        let saved_blocks = persist.load(
                            view_state.offset.x - buffer,
                            view_state.viewport.x + buffer,
                            view_state.offset.y - buffer,
                            view_state.viewport.y + buffer,
                        );

                        let mut ids: Vec<String> = Vec::with_capacity(saved_blocks.len());
                        let mut positions: HashMap<String, BlockPosition> = HashMap::with_capacity(saved_blocks.len());
                        let mut blocks: HashMap<String, Block> = HashMap::with_capacity(saved_blocks.len());

                        for i in 0..saved_blocks.len() {
                            let block = saved_blocks.get(i).unwrap();
                            ids.insert(i, block.id.clone());
                            positions.insert(block.id.clone(), BlockPosition {
                                id: block.id.clone(),
                                x: block.position.x,
                                y: block.position.y,
                                size: Default::default(),
                            });
                            blocks.insert(block.id.clone(), Block {
                                id: block.id.clone(),
                                block_type: block.block_type,
                                block_data: block.block_data.clone(),
                            });
                        }


                        let board_state = BoardState {
                            positions: positions,
                            blocks: blocks,
                            ids: ids,
                            sizes: Default::default()
                        };

                        board_state_sender.send(board_state);
                    }
                    Err(_) => ()
                }
            }
        });

        instance.view_state_sender.insert(view_state_sender);
        instance.board_state_receiver.insert(board_state_receiver);

        return instance;
    }

    pub fn get_interact_point(&self, state: &PointerState) -> Pos2 {
        let interact_point = state.interact_pos();
        if interact_point.is_some() {
            let x = interact_point.unwrap().x + self.view_state.offset.x;
            let y = interact_point.unwrap().y + self.view_state.offset.y;
            return Pos2::new(x, y);
        }
        return Pos2::default();
    }

    pub fn add_label(&mut self, x: f32, y: f32) {
        let id = Uuid::new_v4().to_string();
        self.board_state.ids.push(id.clone());


        let block = Block {
            id: Uuid::new_v4().to_string(),
            block_type: BlockType::Label,
            block_data: String::from("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."),
        };

        self.board_state.blocks.insert(id.clone(), block.clone());

        let position = BlockPosition {
            id: id.clone(),
            x,
            y,
            size: Vec2::new(0.00, 0.00),
        };

        self.board_state.positions.insert(id.clone(), position.clone());

        self.persist.on_add(SavedBlock {
            size: position.size,
            position: Pos2::new(x, y),
            id: id.clone(),
            block_type: block.block_type,
            block_data: block.block_data,
        });
    }

    pub fn on_viewport_change(&mut self) {
        let next = self.last_viewport_change.add(Duration::from_millis(100));

        if Instant::now().lt(&next) {
            return;
        }

        let diff = self.view_state.viewport - self.view_state.last_offset;

        if diff.x >= -BUFFER && diff.x <= BUFFER && diff.y >= -BUFFER && diff.y <= BUFFER {
            return
        }

        self.view_state.last_offset = self.view_state.viewport;
        self.last_viewport_change = Instant::now();
        self.view_state_sender.as_ref().unwrap().send(self.view_state);
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let screen_size = ctx.input().screen_rect().size();
        self.view_state.viewport = screen_size + self.view_state.offset;

        if !self.initialized {
            self.view_state_sender.as_ref().unwrap().send(self.view_state);
            self.initialized = true;
        }

        match self.board_state_receiver.borrow_mut().as_ref().unwrap().try_recv() {
            Ok(value) => {
                self.board_state = value;
            }
            Err(_) => {}
        }

        //ctx.set_debug_on_hover(true);

        if self.pixels_per_point <= 0.00 {
            self.pixels_per_point = ctx.pixels_per_point();
        }

        if self.pixels_per_point > 5.00 {
            self.pixels_per_point = 5.00;
        }

        if self.last_pixels_per_point != self.pixels_per_point {
            ctx.set_pixels_per_point(self.pixels_per_point);
            self.last_pixels_per_point = self.pixels_per_point;
        }

        if ctx.input().key_down(Key::ArrowDown) {
            self.pixels_per_point -= 0.05;
        }

        if ctx.input().key_down(Key::ArrowUp) {
            self.pixels_per_point += 0.05;
        }

        let scroll_delta = ctx.input().scroll_delta;
        if scroll_delta != Vec2::ZERO {
            self.view_state.offset -= scroll_delta;
            self.on_viewport_change();
        }

        let pointer = ctx.input().pointer.clone();

        if !pointer.any_down() {
            self.dragging_widget = String::from("")
        }

        if ctx.input().key_down(Key::Space) {
            let interact_point = self.get_interact_point(&pointer);
            let x = interact_point.x;
            let y = interact_point.y;
            self.add_label(x, y);
        }


        if self.dragging_widget == "" {
            let interact_point = self.get_interact_point(&pointer);
            let x = interact_point.x;
            let y = interact_point.y;

            let mut matched = false;
            for id in &self.board_state.ids {
                let block_position = self.board_state.positions.get(id).unwrap();
                let widget_size = block_position.size;
                if x >= block_position.x && x <= block_position.x + widget_size.x && y >= block_position.y && y <= block_position.y + widget_size.y {
                    if pointer.primary_down() {
                        self.dragging_widget = block_position.id.clone();
                        self.selected_widget = block_position.id.clone();
                    }
                    self.hovered_widget = block_position.id.clone();
                    matched = true;
                }
            }

            if !matched {
                self.hovered_widget = String::new();
                if pointer.any_down() {
                    self.selected_widget = String::new();
                }
            }
        }

        if pointer.any_down() && pointer.is_moving() {
            if self.dragging_widget != "" {
                let position = self.board_state.positions.get_mut(&self.dragging_widget).unwrap();
                position.x += pointer.delta().x;
                position.y += pointer.delta().y;
                self.persist.on_move(&self.dragging_widget, position.x, position.y)
            } else {
                self.view_state.offset.x -= pointer.delta().x;
                self.view_state.offset.y -= pointer.delta().y;
                self.on_viewport_change();
            }
        }


        let mut is_double_click = false;


        if pointer.any_click() {

            let now = Instant::now();

            let diff = now.duration_since(self.last_click);

            if diff.as_millis() < 350 {
                is_double_click = true;
            }

            self.last_click = now;
        }

        if is_double_click && self.hovered_widget == "" {
            let interact_point = self.get_interact_point(&pointer);
            self.add_label(interact_point.x, interact_point.y);
        }

        egui::CentralPanel::default().show(ctx, |ui| {

            self.rendered_blocks = 0;
            self.total_blocks = 0;

            let old_clip_rect = ui.clip_rect();
            ui.set_clip_rect(Rect::NOTHING);

            for id in &self.board_state.ids {

                if self.board_state.sizes.contains_key(id) {
                    continue
                }

                let mut block_position = self.board_state.positions.get_mut(id).unwrap();
                let block = self.board_state.blocks.get_mut(&block_position.id).unwrap();
                match block.block_type {
                    BlockType::Button => {}
                    BlockType::Label => {
                        let r = match &self.selected_widget == id {
                            true => {
                                egui::TextEdit::multiline(&mut block.block_data)
                                    .hint_text("Type something!").ui(ui)
                            }
                            false => {
                                egui::Label::new(block.block_data.clone()).ui(ui)
                            }
                        };

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

                        self.board_state.sizes.insert(block.id.clone(), size);
                    }
                }
            }

            ui.set_clip_rect(old_clip_rect);

            for id in &self.board_state.ids {
                self.total_blocks += 1;
                let mut block_position = self.board_state.positions.get_mut(id).unwrap();
                self.rendered_blocks += 1;
                let position = Pos2::new(block_position.x - self.view_state.offset.x, block_position.y - self.view_state.offset.y);
                let block = self.board_state.blocks.get_mut(&block_position.id).unwrap();
                let original_data = block.block_data.clone();

                match block.block_type {
                    BlockType::Button => {
                        // ui.put(widget_rect, egui::Button::new(data));
                    }
                    BlockType::Label => {

                        match self.board_state.sizes.get(id) {
                            None => {}
                            Some(size) => {
                                let widget_rect = Rect::from_min_size(position, *size);

                                ui.set_clip_rect(old_clip_rect);

                                let r2 = match &self.selected_widget == id {
                                    true => {
                                        ui.put(widget_rect, egui::TextEdit::multiline(&mut block.block_data)
                                            .hint_text("Type something!"))
                                    }
                                    false => {
                                        ui.put(widget_rect, egui::Label::new(block.block_data.clone()).wrap(true))
                                    }
                                };

                                // if block_position.size != r2.rect.size() {
                                //     block_position.size = r2.rect.size();
                                //     println!("block size change: {}, {}", r2.rect.size().x, r2.rect.size().y);
                                //     self.persist.on_size_change(id, r2.rect.size());
                                // }

                                if id == &self.selected_widget {
                                    ui.painter().rect_stroke(r2.rect, 4.0, (1.0, Color32::RED));

                                    if original_data != block.block_data {
                                        self.persist.on_data_change(id, block.block_data.clone());
                                    }
                                }

                                else if id == &self.hovered_widget {
                                    ui.painter().rect_stroke(r2.rect, 4.0, (1.0, Color32::LIGHT_BLUE));
                                }
                            }
                        }
                    }
                }
            }
        });

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

        if self.debug_mode {
            egui::SidePanel::left("side_panel").show(ctx, |ui| {

                ui.heading("Side Panel");

                ui.add(egui::Slider::new(&mut self.pixels_per_point, 0.0..=10.00).text("Pixels Per Point"));

                if self.selected_widget != "" {
                    match self.board_state.positions.get_mut(&self.selected_widget) {
                        None => {}
                        Some(block_position) => {
                            ui.add(egui::Slider::new(&mut block_position.size.x, 0.0..=1000.00).text("Selected Widget Width"));
                            ui.add(egui::Slider::new(&mut block_position.size.y, 0.0..=1000.00).text("Selected Widget Height"));
                        }
                    }
                }

                ui.label(format!("Screen Offset: {}, {}", self.view_state.offset.x, self.view_state.offset.y));
                ui.label(format!("Screen Size: {}, {}", screen_size.x, screen_size.y));

                let clip_rect = ui.clip_rect();

                ui.label(format!("Viewport: {} - {}, {} - {}", self.view_state.offset.x, self.view_state.viewport.x, self.view_state.offset.y, self.view_state.viewport.y));

                ui.label(format!("Dragging Widget: {}", self.dragging_widget));

                ui.label(format!("Clip Rect: {}, {}", clip_rect.size().x, clip_rect.size().y));

                ui.label(format!("Rendered Blocks: {}", self.rendered_blocks));
                ui.label(format!("Total Blocks: {}", self.total_blocks));
            });
        }
    }
}