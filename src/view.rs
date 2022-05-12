use egui::Vec2;

pub struct ViewState {
    pub(crate) viewport: Vec2,
    pub(crate) screen: Vec2,
}

impl Default for ViewState {
    fn default() -> Self {
        return Self {
            viewport: Vec2::ZERO,
            screen: Vec2::ZERO
        }
    }
}

impl ViewState {

    pub fn in_viewport(&self, x: f32, y: f32) -> bool {
        let buffer = 300.00;
        let y_min = self.screen.y - buffer;
        let y_max = self.viewport.y + buffer;
        let x_min = self.screen.x;
        let x_max = self.viewport.x + buffer - buffer;

        if y > y_max || y < y_min || x > x_max || x < x_min {
            return false;
        }

        return true;
    }

}