/// Showcase [`TextEdit`].
#[derive(PartialEq)]
pub struct TextEdit {
    pub text: String,
}

impl Default for TextEdit {
    fn default() -> Self {
        Self {
            text: "Edit this text".to_owned(),
        }
    }
}
impl super::Block for TextEdit {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { text } = self;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Advanced usage of ");
            ui.code("TextEdit");
            ui.label(".");
        });

        let output = egui::TextEdit::multiline(text)
            .hint_text("Type something!")
            .show(ui);



        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Selected text: ");
            if let Some(text_cursor_range) = output.cursor_range {
                use egui::TextBuffer as _;
                let selected_chars = text_cursor_range.as_sorted_char_range();
                let selected_text = text.char_range(selected_chars);
                ui.code(selected_text);
            }
        });

        let anything_selected = output
            .cursor_range
            .map_or(false, |cursor| !cursor.is_empty());

        ui.add_enabled(
            anything_selected,
            egui::Label::new("Press ctrl+T to toggle the case of selected text (cmd+T on Mac)"),
        );

        if ui
            .input_mut()
            .consume_key(egui::Modifiers::COMMAND, egui::Key::T)
        {
            if let Some(text_cursor_range) = output.cursor_range {
                use egui::TextBuffer as _;
                let selected_chars = text_cursor_range.as_sorted_char_range();
                let selected_text = text.char_range(selected_chars.clone());
                let upper_case = selected_text.to_uppercase();
                let new_text = if selected_text == upper_case {
                    selected_text.to_lowercase()
                } else {
                    upper_case
                };
                text.delete_char_range(selected_chars.clone());
                text.insert_text(&new_text, selected_chars.start);
            }
        }

        ui.horizontal(|ui| {
            ui.label("Move cursor to the:");

            if ui.button("start").clicked() {
                let text_edit_id = output.response.id;
                if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                    let ccursor = egui::text::CCursor::new(0);
                    state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
                    state.store(ui.ctx(), text_edit_id);
                    ui.ctx().memory().request_focus(text_edit_id); // give focus back to the [`TextEdit`].
                }
            }

            if ui.button("end").clicked() {
                let text_edit_id = output.response.id;
                if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                    let ccursor = egui::text::CCursor::new(text.chars().count());
                    state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
                    state.store(ui.ctx(), text_edit_id);
                    ui.ctx().memory().request_focus(text_edit_id); // give focus back to the [`TextEdit`].
                }
            }
        });
    }
}