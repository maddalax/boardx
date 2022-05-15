use egui::Vec2;

#[derive(Clone, Copy)]
pub struct ViewState {
    pub(crate) viewport: Vec2,
    pub(crate) offset: Vec2,
}

impl Default for ViewState {
    fn default() -> Self {
        return Self {
            viewport: Vec2::ZERO,
            offset: Vec2::ZERO
        }
    }
}

impl ViewState {

    pub fn in_viewport(&self, x: f32, y: f32) -> bool {
        let buffer = 300.00;
        let y_min = self.offset.y - buffer;
        let y_max = self.viewport.y + buffer;
        let x_min = self.offset.x - buffer;
        let x_max = self.viewport.x + buffer;

        if y > y_max || y < y_min || x > x_max || x < x_min {
            return false;
        }

        return true;
    }

}