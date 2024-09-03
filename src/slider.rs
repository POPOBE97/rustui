use egui::{vec2, Color32, DragValue, Layout, Rect, Sense, Widget};

pub struct Slider<'a> {
  pub tint: &'a str,
  pub range: std::ops::RangeInclusive<f64>,
  pub get_set_value: Box<dyn 'a + FnMut(Option<f64>) -> f64>,
}

impl<'a> Slider<'a> {
  pub fn from_get_set(range: std::ops::RangeInclusive<f64>, tint: &'a str, get_set_value: impl 'a + FnMut(Option<f64>) -> f64) -> Self {
    Self { tint: &tint, range, get_set_value: Box::new(get_set_value) }
  }
}

impl<'a> Widget for Slider<'a> {
  fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
    ui.allocate_ui_with_layout(ui.available_size_before_wrap(), Layout::right_to_left(egui::Align::Center), |ui| {
      // 1.draw the drag value
      let mut value = (self.get_set_value)(None);
      let available_size = ui.available_size_before_wrap();
      ui.add_sized(vec2(0.0, available_size.y),DragValue::new(&mut value).speed(0.1).custom_formatter(|r, _| {
        if r.abs() < 0.1 {
          format!("{:.2}", r)
        } else if r.abs() < 1.0 {
          format!("{:.1}", r)
        } else {
          format!("{:.0}", r)
        }
      }));

      let mut available_size = ui.available_size_before_wrap();
      available_size.x = available_size.x.max(ui.style().spacing.slider_width);

      let (mut response, painter) = ui
        .allocate_painter(available_size, Sense::click_and_drag());
      response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

      let rect = painter.clip_rect();
      let size = rect.max - rect.min;

      // 1. draw background rectangle
      let background_alpha = if response.hovered() { "05" } else { "01" };
      painter.rect_filled(rect, 4.0, Color32::from_hex(format!("#{}{}", self.tint, background_alpha).as_str()).unwrap());

      // 2. draw the cursor
      // if response.hovered() {
        let cursor_size = vec2(4.0, size.y);
        let mut per = (value - self.range.start()) / (self.range.end() - self.range.start());
        per = per.min(1.0).max(0.0);
        let pos = rect.min + vec2((per as f32) * (size.x - cursor_size.x), 0.0);
        let cursor_rect = Rect { min: pos, max: pos + cursor_size };
        let cursor_alpha = if response.hovered() { "FF" } else { "0A" };
        painter.rect_filled(cursor_rect, 4.0, Color32::from_hex(format!("#{}{}", self.tint, cursor_alpha).as_str()).unwrap());
      // }

      if response.is_pointer_button_down_on() {
        let mut per = (response.interact_pointer_pos().unwrap().x - rect.min.x) / size.x;
        per = per.min(1.0).max(0.0);
        value = self.range.start() + (self.range.end() - self.range.start()) * (per as f64);
      }

      (self.get_set_value)(Some(value));
    }).response
  }
}