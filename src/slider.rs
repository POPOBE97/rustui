use eframe::egui::{emath::Numeric, vec2, Color32, DragValue, Layout, Rect, Sense, Widget};

pub struct Slider<'a, T> {
  pub title: Option<&'a str>,
  pub range: std::ops::RangeInclusive<T>,
  pub get_set_value: Box<dyn 'a + FnMut(Option<T>) -> T>,
}

impl<'a, T> Slider<'a, T> {
  pub fn from_get_set(range: std::ops::RangeInclusive<T>, get_set_value: impl 'a + FnMut(Option<T>) -> T) -> Self {
    Self { range, get_set_value: Box::new(get_set_value), title: None }
  }

  pub fn with_title(mut self, title: &'a str) -> Self {
    self.title = Some(title);
    self
  }
}

impl<'a, T: Numeric> Widget for Slider<'a, T> {
  fn ui(mut self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
    let mut available_size = ui.available_size_before_wrap();
    available_size.y = 20.0;
    ui.allocate_ui_with_layout(available_size, Layout::right_to_left(eframe::egui::Align::Center), |ui| {
      // 1.draw the drag value
      let mut value = (self.get_set_value)(None);
      ui.add_sized(vec2(0.0, available_size.y),DragValue::new(&mut value).speed(0.1).custom_formatter(|r, _| {
        if r.abs() < 1.0 {
          format!("{:.3}", r)
        } else if r.abs() < 10.0 {
          format!("{:.2}", r)
        } else if r.abs() < 100.0 {
          format!("{:.1}", r)
        } else {
          format!("{:.0}", r)
        }
      }));

      let mut available_size = ui.available_size_before_wrap();
      available_size.x = available_size.x.max(ui.style().spacing.slider_width);
      if self.title.is_some() {
        available_size.x -= 108.0;
      }

      let (mut response, painter) = ui
        .allocate_painter(available_size, Sense::click_and_drag());
      response = response.on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

      let rect = painter.clip_rect();
      let size = rect.max - rect.min;

      // 1. draw background rectangle
      let background_alpha = if response.hovered() { 
        if ui.visuals().dark_mode { "05" } else { "10" }
      } else {
        if ui.visuals().dark_mode { "01" } else { "06" }
      };
      let tint = if ui.visuals().dark_mode { "FFFFFF" } else { "000000" };
      painter.rect_filled(rect, 4.0, Color32::from_hex(format!("#{}{}", tint, background_alpha).as_str()).unwrap());

      if let Some(title) = self.title {
        ui.allocate_ui_with_layout(vec2(100.0, 20.0), Layout::left_to_right(eframe::egui::Align::Center), |ui| {
          ui.label(title);
        });
      }
      // 2. draw the cursor
      let cursor_size = vec2(4.0, size.y);
      let start = self.range.start().to_f64();
      let end = self.range.end().to_f64();
      let v = value.to_f64();
      let mut per = (v - start) / (end - start);
      
      per = per.min(1.0).max(0.0);
      let pos = rect.min + vec2((per as f32) * (size.x - cursor_size.x), 0.0);
      let cursor_rect = Rect { min: pos, max: pos + cursor_size };
      let cursor_alpha = if response.hovered() { 
        if ui.visuals().dark_mode { "FF" } else { "CC" }
      } else {
        if ui.visuals().dark_mode { "0A" } else { "18" }
      };
      painter.rect_filled(cursor_rect, 4.0, Color32::from_hex(format!("#{}{}", tint, cursor_alpha).as_str()).unwrap());

      if response.is_pointer_button_down_on() {
        let mut per = (response.interact_pointer_pos().unwrap().x - rect.min.x) / size.x;
        per = per.min(1.0).max(0.0);
        value = T::from_f64(start + (end - start) * per as f64);
      }

      (self.get_set_value)(Some(value));
    }).response
  }
}