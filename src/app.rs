
use std::fs;
use std::fs::File;
use std::io::Read;

use eframe::emath::{Align2, Vec2};
use eframe::epaint::{Color32, Rgba};

use egui_extras::RetainedImage;

use egui::{Id, LayerId, Order, TextStyle, Visuals, Widget, DroppedFile, ColorImage, Rect, Pos2, FontId, Layout, Sense};
use egui::Shape::Vec;
use crate::demo::Block;
use crate::demo::text_edit::TextEdit;

pub struct TemplateApp {
    // Example stuff:
    label: String,

    value: f32,

    editor: TextEdit,

    image: Option<Result<RetainedImage, String>>,

    offset: Vec2,

    offset2: Vec2,

    screen: Vec2,

    dragging_widget: bool
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            editor: TextEdit::default(),
            image: None,
            offset: Vec2::new(800.00, 1500.00),
            offset2: Vec2::new(300.00, 800.00),
            screen: Vec2::new(0.00, 0.00),
            dragging_widget: false,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for TemplateApp {



    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value, editor, image, offset, offset2, screen, .. } = self;

        ctx.set_debug_on_hover(true);

        let size = ctx.input().screen_rect().size();
        let min = Pos2::new(offset.x - screen.x, offset.y - screen.y);
        let min2 = Pos2::new(offset2.x - screen.x, offset2.y - screen.y);


        let pointer = ctx.input().pointer.clone();



        if pointer.any_down() && pointer.interact_pos().is_some() {
            let interact_point = pointer.interact_pos().unwrap();
            let x = interact_point.x + screen.x;
            let y = interact_point.y + screen.y;
            let widget_size = Vec2::new(100.00, 100.00);

            println!("X: {}, Y: {}, Min X: {}, Min Y: {}, Max X: {}, Max Y: {}", x, y, offset.x, offset.y, offset.x + widget_size.x, offset.y + widget_size.y);

            if x >= offset.x && x <= offset.x + widget_size.x && y >= offset.y && y <= offset.y + widget_size.y {
                self.dragging_widget = true;
            }

        }

        if pointer.any_down() && pointer.is_moving() {
            if self.dragging_widget {
                offset.x += pointer.delta().x;
                offset.y += pointer.delta().y
            } else {
                screen.x -= pointer.delta().x;
                screen.y -= pointer.delta().y;
            }
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

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(&mut screen.x, 0.0..=size.x * 4.00).text("Screen X Position"));
            ui.add(egui::Slider::new(&mut screen.y, 0.0..=size.y * 4.00).text("Screen Y Position"));

            ui.label(format!("Screen Offset: {}, {}", screen.x, screen.y));
            ui.label(format!("Screen Size: {}, {}", size.x, size.y));

            let area = size + *screen;

            ui.label(format!("Area: {}, {}", area.x, area.y));
            ui.label(format!("Item Offset: {}, {}", offset.x, offset.y));
            ui.label(format!("Item Within Area: {}, {}", min.x, min.y));

            ui.label(format!("Dragging Widget: {}", self.dragging_widget));

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            let widget_rect =
                Rect::from_min_size(min, Vec2::new(100.00, 100.00));

            let widget_rect2 =
                Rect::from_min_size(min2, Vec2::new(100.00, 100.00));


            ui.put(widget_rect, egui::Button::new("My Button"));

            ui.put(widget_rect2, egui::Button::new("My Button 2"));


        });
    }
}