pub mod text_edit;

pub trait Block {
    fn ui(&mut self, ui: &mut egui::Ui);
}