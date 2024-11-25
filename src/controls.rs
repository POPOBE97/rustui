use std::{collections::BTreeMap, ops::RangeInclusive};
use crate::slider::Slider;

pub enum ControlValue {
  Int(i32, bool),
  Bool(bool, bool),
  Float(f32, bool),
  Vec2([f32; 2], bool),
  Vec3([f32; 3], bool),
  Vec4([f32; 4], bool),
}

#[macro_export]
macro_rules! control_map {
  // Empty map
  () => {
    BTreeMap::<&str, (ControlValue, bool)>::new()
  };
  
  // Map with key-value pairs
  ($($key:expr => $value:expr),* $(,)?) => {
    {
      let mut map = BTreeMap::<&str, (ControlValue, bool)>::new();
      $(
        map.insert($key, ($value, true));
      )*
      map
    }
  };
}

pub struct Controls {
  values: BTreeMap<&'static str, ControlGroup>,
}

impl Controls {
  pub fn new(default_values: BTreeMap<&'static str, ControlGroup>) -> Controls {
    Controls {
      values: default_values,
    }
  }

  // pub fn float(&mut self, ui: &mut eframe::egui::Ui, name: &'a str, range: RangeInclusive<f32>) -> &mut Self {
  //   let pair = self.values.get_mut(name).unwrap();
  //   let needs_update = &mut pair.1;
  //   let value = match &mut pair.0 {
  //     ControlValue::Float(v, _, _) => v,
  //     _ => panic!("{} type is not float", name),
  //   };

  //   ui.add(Slider::from_get_set(range, |v| {
  //     if let Some(v) = v {
  //       *value = v;
  //       *needs_update = true;
  //     }
  //     return *value;
  //   }).with_title(name));

  //   self
  // }

  // pub fn bool(&mut self, ui: &mut eframe::egui::Ui, name: &'a str) -> &mut Self {
  //   let pair = self.values.get_mut(name).unwrap();
  //   let needs_update = &mut pair.1;
  //   let value = match &mut pair.0 {
  //     ControlValue::Bool(v, _, _) => v,
  //     _ => panic!("{} type is not bool", name),
  //   };

  //   ui.add(Slider::from_get_set(0..=1, |v| {
  //     if let Some(v) = v {
  //       *value = v == 1;
  //       *needs_update = true;
  //     }
  //     return if *value { 1 } else { 0 };
  //   }).with_title(name));

  //   self
  // }

  // pub fn vec2(&mut self, ui: &mut eframe::egui::Ui, name: &'a str, r1: RangeInclusive<f32>, r2: RangeInclusive<f32>) -> &mut Self {
  //   let pair = self.values.get_mut(name).unwrap();
  //   let needs_update = &mut pair.1;
  //   let value = match &mut pair.0 {
  //     ControlValue::Vec2(v, _, _) => v,
  //     _ => panic!("{} type is not vec2", name),
  //   };

  //   ui.add(Slider::from_get_set(r1, |v| {
  //     if let Some(v) = v {
  //       (*value)[0] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[0];
  //   }).with_title(format!("{}.x", name).as_str()));

  //   ui.add(Slider::from_get_set(r2, |v| {
  //     if let Some(v) = v {
  //       (*value)[1] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[1];
  //   }).with_title(format!("{}.y", name).as_str()));

  //   self
  // }

  // pub fn vec3(&mut self, ui: &mut eframe::egui::Ui, name: &'a str, r1: RangeInclusive<f32>, r2: RangeInclusive<f32>, r3: RangeInclusive<f32>) -> &mut Self {
  //   let pair = self.values.get_mut(name).unwrap();
  //   let needs_update = &mut pair.1;
  //   let value = match &mut pair.0 {
  //     ControlValue::Vec3(v, _, _) => v,
  //     _ => panic!("{} type is not vec3", name),
  //   };

  //   ui.add(Slider::from_get_set(r1, |v| {
  //     if let Some(v) = v {
  //       (*value)[0] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[0];
  //   }).with_title(format!("{}.x", name).as_str()));

  //   ui.add(Slider::from_get_set(r2, |v| {
  //     if let Some(v) = v {
  //       (*value)[1] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[1];
  //   }).with_title(format!("{}.y", name).as_str()));

  //   ui.add(Slider::from_get_set(r3, |v| {
  //     if let Some(v) = v {
  //       (*value)[2] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[2];
  //   }).with_title(format!("{}.z", name).as_str()));

  //   self
  // }

  // pub fn vec4(&mut self, ui: &mut eframe::egui::Ui, name: &'a str, r1: RangeInclusive<f32>, r2: RangeInclusive<f32>, r3: RangeInclusive<f32>, r4: RangeInclusive<f32>) -> &mut Self {
  //   let pair = self.values.get_mut(name).unwrap();
  //   let needs_update = &mut pair.1;
  //   let value = match &mut pair.0 {
  //     ControlValue::Vec4(v, _, _) => v,
  //     _ => panic!("{} type is not vec4", name),
  //   };    

  //   ui.add(Slider::from_get_set(r1, |v| {
  //     if let Some(v) = v {
  //       (*value)[0] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[0];
  //   }).with_title(format!("{}.x", name).as_str()));

  //   ui.add(Slider::from_get_set(r2, |v| {
  //     if let Some(v) = v {
  //       (*value)[1] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[1];
  //   }).with_title(format!("{}.y", name).as_str()));    

  //   ui.add(Slider::from_get_set(r3, |v| {
  //     if let Some(v) = v {
  //       (*value)[2] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[2];
  //   }).with_title(format!("{}.z", name).as_str()));

  //   ui.add(Slider::from_get_set(r4, |v| {
  //     if let Some(v) = v {
  //       (*value)[3] = v;
  //       *needs_update = true;
  //     }
  //     return (*value)[3];
  //   }).with_title(format!("{}.w", name).as_str()));

  //   self
  // }

  pub fn to_json(&self) -> String {
    // let json = serde_json::to_string(&self.values).unwrap();
    // let json_string = serde_json::to_string_pretty(&json).unwrap();

    todo!()
  }

  pub fn save_json(&self) {
    // let js_value = rfd::AsyncFileDialog::new()
    //   .set_file_name("uniforms.json")
    //   .save_file();
    
    // // if is web
    // #[cfg(target_arch = "wasm32")] {
    //   // Spawn the save dialog asynchronously
    //   wasm_bindgen_futures::spawn_local(async move {
    //     if let Some(file) = js_value.await {
    //       // File was saved successfully
    //       file.write(json_string.as_bytes()).await.unwrap();
    //     }
    //   });
    // }

    // // if is native
    // #[cfg(not(target_arch = "wasm32"))] {
    //   // Spawn the save dialog asynchronously
    //   tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .spawn(async move {
    //       println!("saving");
    //       if let Some(file) = js_value.await {
    //         println!("file: {:?}", file);
    //         // File was saved successfully
    //         file.write(json_string.as_bytes()).await.unwrap();
    //       }
    //     });
    // }
    todo!()
  }

  pub fn group(&mut self, name: &'static str, build: impl FnOnce(ControlGroupBuilder) -> ControlGroupBuilder) -> &mut Self {
    if !self.values.contains_key(name) {
      self.values.insert(name, ControlGroup {
        name,
        values: BTreeMap::new(),
        needs_update: false,
      });
    }

    let group = self.values.get_mut(name).unwrap();
    let builder = ControlGroupBuilder::new(group);

    build(builder);
    
    self
  }
}


// ---------- group builder ---------- //

pub struct ControlGroup {
  pub name: &'static str, 
  pub values: BTreeMap<&'static str, ControlValue>,
  pub needs_update: bool,
}

pub struct ControlGroupBuilder<'a> {
  group: &'a mut ControlGroup,
}

impl<'a> ControlGroupBuilder<'a> {
  pub fn new(group: &'a mut ControlGroup) -> ControlGroupBuilder<'a> {
    ControlGroupBuilder {
      group,
    }
  }

  pub fn int(self, ui: &mut eframe::egui::Ui, name: &'static str, r: RangeInclusive<i32>) -> Self {
    // if !self.group.values.contains_key(&name) {
    //   self.group.values.insert(name, ControlValue::Int(value, true));
    // };

    let (value_ref, need_update_ref) = {
      let (v, u) = match self.group.values.get_mut(name).unwrap() {
        ControlValue::Int(v, u) => (v, u),
        _ => panic!("{} type is not int", name),
      };
      (v, u)
    };

    ui.add(Slider::from_get_set(r, |v| {
      if let Some(v) = v {
        *value_ref = v;
        *need_update_ref = true;
      }
      return *value_ref;
    }).with_title(name));

    self
  }

  pub fn float(self, ui: &mut eframe::egui::Ui, name: &'static str, r: RangeInclusive<f32>) -> Self {
    // if !self.group.values.contains_key(&name) {
    //   self.group.values.insert(name, ControlValue::Float(value, true));
    // };

    let (value_ref, need_update_ref) = {
      let (v, u) = match self.group.values.get_mut(name).unwrap() {
        ControlValue::Float(v, u) => (v, u),
        _ => panic!("{} type is not float", name),
      };
      (v, u)
    };

    ui.add(Slider::from_get_set(r, |v| {
      if let Some(v) = v {
        *value_ref = v;
        *need_update_ref = true;
      }
      return *value_ref;
    }).with_title(name));

    self
  }

  pub fn vec2(self, ui: &mut eframe::egui::Ui, name: &'static str, r1: RangeInclusive<f32>, r2: RangeInclusive<f32>) -> Self {
    // if !self.group.values.contains_key(&name) {
    //   self.group.values.insert(name, ControlValue::Vec2(value, true));
    // };

    let (value_ref, need_update_ref) = {
      let (v, u) = match self.group.values.get_mut(name).unwrap() {
        ControlValue::Vec2(v, u) => (v, u),
        _ => panic!("{} type is not vec2", name),
      };
      (v, u)
    };

    ui.add(Slider::from_get_set(r1, |v| {
      if let Some(v) = v {
        (*value_ref)[0] = v;
        *need_update_ref = true;
      }
      return (*value_ref)[0];
    }).with_title(format!("{}.x", name).as_str()));

    ui.add(Slider::from_get_set(r2, |v| {
      if let Some(v) = v {
        (*value_ref)[1] = v;
        *need_update_ref = true;
      }
      return (*value_ref)[1];
    }).with_title(format!("{}.y", name).as_str()));

    self
  }
}

