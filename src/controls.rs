use std::{collections::BTreeMap, ops::{Deref, Index, IndexMut, RangeInclusive}};

use crate::{persist_order_map::PersistOrderMap, slider::Slider};

#[derive(Debug)]
pub enum ControlValue {
  Int(i32, Vec<u8>, bool),
  Bool(i32, Vec<u8>, bool),
  Float(f32, Vec<u8>, bool),
  Vec2([f32; 2], Vec<u8>, bool),
  Vec3([f32; 3], Vec<u8>, bool),
  Vec4([f32; 4], Vec<u8>, bool),
}

impl ControlValue {
  pub fn size(&self) -> usize {
    match self {
      ControlValue::Int(_, _, _) => std::mem::size_of::<i32>(),
      ControlValue::Bool(_, _, _) => std::mem::size_of::<i32>(),
      ControlValue::Float(_, _, _) => std::mem::size_of::<f32>(),
      ControlValue::Vec2(_, _, _) => std::mem::size_of::<f32>() * 2,
      ControlValue::Vec3(_, _, _) => std::mem::size_of::<f32>() * 3,
      ControlValue::Vec4(_, _, _) => std::mem::size_of::<f32>() * 4,
    }
  }

  pub fn get_bytes(&self) -> &[u8] {
    match self {
      ControlValue::Int(_, b, _) => b,
      ControlValue::Bool(_, b, _) => b,
      ControlValue::Float(_, b, _) => b,
      ControlValue::Vec2(_, b, _) => b,
      ControlValue::Vec3(_, b, _) => b,
      ControlValue::Vec4(_, b, _) => b,
    }
  }
}

impl From<i32> for ControlValue {
  fn from(value: i32) -> Self { ControlValue::Int(value, value.to_ne_bytes().to_vec(), true) }
}

impl From<bool> for ControlValue {
  fn from(value: bool) -> Self { 
    let value = if value { 1 } else { 0 };
    ControlValue::Bool(value, value.to_ne_bytes().to_vec(), true) 
  }
}

impl From<f32> for ControlValue {
  fn from(value: f32) -> Self { 

    ControlValue::Float(value, value.to_ne_bytes().to_vec(), true) 
  }
}

impl From<[f32; 2]> for ControlValue {
  fn from(value: [f32; 2]) -> Self {
    let mut bytes = [0u8; 8];
    bytes[..4].copy_from_slice(&value[0].to_ne_bytes());
    bytes[4..].copy_from_slice(&value[1].to_ne_bytes());
    ControlValue::Vec2(value, bytes.to_vec(), true) 
  }
}

impl From<[f32; 3]> for ControlValue {
  fn from(value: [f32; 3]) -> Self {
    let mut bytes = [0u8; 12];
    bytes[..4].copy_from_slice(&value[0].to_ne_bytes());
    bytes[4..8].copy_from_slice(&value[1].to_ne_bytes());
    bytes[8..].copy_from_slice(&value[2].to_ne_bytes());
    ControlValue::Vec3(value, bytes.to_vec(), true) 
  }
}

impl From<[f32; 4]> for ControlValue {
  fn from(value: [f32; 4]) -> Self {
    let mut bytes = [0u8; 16];
    bytes[..4].copy_from_slice(&value[0].to_ne_bytes());
    bytes[4..8].copy_from_slice(&value[1].to_ne_bytes());
    bytes[8..12].copy_from_slice(&value[2].to_ne_bytes());
    bytes[12..].copy_from_slice(&value[3].to_ne_bytes());
    ControlValue::Vec4(value, bytes.to_vec(), true) 
  }
}


// -------------------- group builder -------------------- //
#[derive(Debug)]
pub struct ControlGroup {
  pub name: &'static str, 
  pub values: PersistOrderMap<&'static str, ControlValue>,
  pub packed: Vec<u8>,
  pub needs_update: bool,
  pub size: usize,
}

impl ControlGroup {
  pub fn new(name: &'static str) -> ControlGroup {
    ControlGroup {
      name,
      values: PersistOrderMap::new(),
      packed: Vec::new(),
      needs_update: false,
      size: 0,
    }
  }
  
  pub fn aligned_size(&self) -> usize {
    let pad = 16 - (self.size % 16);
    self.size + pad
  }

  pub fn get_bytes(&mut self) -> &[u8] {
    if self.needs_update {
      self.pack();
      self.needs_update = false;
    }
    &self.packed
  }

  fn pack(&mut self) {
    let mut package = Vec::new();
    for (_, value) in &self.values {
      match value {
        ControlValue::Int(_, b, _) => {
          package.extend_from_slice(b);
        },
        ControlValue::Bool(_, b, _) => {
          package.extend_from_slice(b);
        },
        ControlValue::Float(_, b, _) => {
          package.extend_from_slice(b);
        },
        ControlValue::Vec2(_, b, _) => {
          package.extend_from_slice(b);
        },
        ControlValue::Vec3(_, b, _) => {
          package.extend_from_slice(b);
        },
        ControlValue::Vec4(_, b, _) => {
          package.extend_from_slice(b);
        }
      }
    }

    // pad to 4 bytes
    let pad = self.aligned_size() - self.size;
    for _ in 0..pad { package.push(0) }
    self.packed = package;
  }
}

impl Index<&'static str> for ControlGroup {
  type Output = ControlValue;
  fn index(&self, name: &'static str) -> &Self::Output { self.values.get(name).unwrap() }
}

impl IndexMut<&'static str> for ControlGroup {
  fn index_mut(&mut self, name: &'static str) -> &mut Self::Output { self.values.get_mut(name).unwrap() }
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

  pub fn int(self, ui: &mut eframe::egui::Ui, name: &'static str, default: i32, r: RangeInclusive<i32>) -> Self {
    if !self.group.values.inner.contains_key(&name) {
      self.group.values.insert(name, ControlValue::from(default));
      self.group.needs_update = true;
    };

    let value = self.group.values.get_mut(name).unwrap();
    let mut value = match value {
      ControlValue::Int(v, _, _) => *v,
      _ => panic!("value is not an int"),
    };

    ui.add(Slider::from_get_set(r, |v| {
      if let Some(v) = v {
        if v == value { return value; }
        value = v;
        self.group.values.insert(name, ControlValue::from(value));
        self.group.needs_update = true;
      }
      return value;
    }).with_title(name));

    self
  }

  pub fn float(self, ui: &mut eframe::egui::Ui, name: &'static str, default: f32, r: RangeInclusive<f32>) -> Self {
    if !self.group.values.inner.contains_key(&name) {
      self.group.values.insert(name, ControlValue::from(default));
      self.group.needs_update = true;
    }

    let value = self.group.values.get_mut(name).unwrap();
    let mut value = match value {
      ControlValue::Float(v, _, _) => *v,
      _ => panic!("value is not a float"),
    };

    ui.add(Slider::from_get_set(r, |v| {
      if let Some(v) = v {
        if v == value { return value; }
        value = v;
        self.group.values.insert(name, ControlValue::from(value));
        self.group.needs_update = true;
        println!("{} = {}", name, value);
      }
      return value;
    }).with_title(name));

    self
  }

  pub fn vec2(self, ui: &mut eframe::egui::Ui, name: &'static str, default: [f32; 2], r1: RangeInclusive<f32>, r2: RangeInclusive<f32>) -> Self {
    if !self.group.values.inner.contains_key(&name) {
      self.group.values.insert(name, ControlValue::from(default));
      self.group.needs_update = true;
    };

    let value = self.group.values.get_mut(name).unwrap();
    let mut value = match value {
      ControlValue::Vec2(v, _, _) => *v,
      _ => panic!("value is not a vec2"),
    };

    ui.add(Slider::from_get_set(r1, |v| {
      if let Some(v) = v {
        if v == value[0] { return value[0]; }
        value[0] = v;
        self.group.values.insert(name, ControlValue::from(value));
        self.group.needs_update = true;
      }
      return value[0];
    }).with_title(format!("{}.x", name).as_str()));

    ui.add(Slider::from_get_set(r2, |v| {
      if let Some(v) = v {
        if v == value[1] { return value[1]; }
        value[1] = v;
        self.group.values.insert(name, ControlValue::from(value));
        self.group.needs_update = true;
      }
      return value[1];
    }).with_title(format!("{}.y", name).as_str()));

    self
  }


  pub fn button(self, ui: &mut eframe::egui::Ui, title: &'static str, mut action: impl FnMut()) -> Self {
      if ui.button(title).clicked() {
        action();
      }
      self
    }

}

// -------------------- action group -------------------- //
pub struct ActionGroup {
  pub name: &'static str,
  pub actions: BTreeMap<&'static str, Box<dyn FnMut(&mut Controls)>>,
}

impl ActionGroup {
  pub fn new(name: &'static str) -> ActionGroup {
    ActionGroup {
      name,
      actions: BTreeMap::new(),
    }
  }
}


pub struct ActionGroupBuilder<'a> {
  group: &'a mut ActionGroup,
}

impl<'a> ActionGroupBuilder<'a> {
  pub fn new(group: &'a mut ActionGroup) -> ActionGroupBuilder<'a> {
    ActionGroupBuilder {
      group,
    }
  }

  pub fn button(self, name: &'static str, action: impl FnMut(&mut Controls) + 'static) -> Self {
    self.group.actions.insert(name, Box::new(action));
    self
  }
}
// -------------------- controls -------------------- //
#[macro_export]
macro_rules! controls {
    () => { Controls::new(BTreeMap::<&str, ControlGroup>::new()) };
    ($($group_name:literal: $group:tt),* $(,)?) => {
      {
        let mut map = BTreeMap::<&str, ControlGroup>::new();
        $(
          map.insert($group_name, controls!($group_name, $group));
        )*
        Controls::new(map)
      }
    };
    ($group_name:literal, {$($name:literal: $properties:tt),* $(,)?}) => {
      {
        let mut map = ControlGroup::new($group_name);
        $(
          // let control_value = ControlValue::from($value);
          // map.size += control_value.size();
          // map.values.insert($name, control_value);
          controls!($group_name, $name, $properties);
        )*
        map.needs_update = true;
        map
      }
    };
    ($group_name:literal, $name:literal, {$($key:literal: $value:tt),* $(,)?}) => {
      
    }
}

#[derive(Debug)]
pub struct Controls {
  values: BTreeMap<&'static str, ControlGroup>,
}

impl Controls {
  pub fn new() -> Controls {
    Controls {
      values: BTreeMap::new(),
    }
  }

  pub fn to_json(&self) -> String {
    // let json = serde_json::to_string(&self.values).unwrap();
    // let json_string = serde_json::to_string_pretty(&json).unwrap();

    todo!()
  }

  pub fn save_json(&mut self) {
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
        values: PersistOrderMap::new(),
        packed: Vec::new(),
        needs_update: false,
        size: 0,
      });
    }

    let group = self.values.get_mut(name).unwrap();
    let builder = ControlGroupBuilder::new(group);

    build(builder);
    
    self
  }

  pub fn action_group(&mut self, ui: &mut eframe::egui::Ui, name: &'static str, build: impl FnOnce(ActionGroupBuilder) -> ActionGroupBuilder) -> &mut Self {
    let mut action_group = ActionGroup::new(name);
    build(ActionGroupBuilder::new(&mut action_group));
    ui.horizontal(|ui| {
      for (title, action) in &mut action_group.actions {
        if ui.button(*title).clicked() {
          action(self);
        }
      }
    });

    ui.add_space(20.0);
    ui.separator();
    self
  }

  pub fn get(&self, name: &'static str) -> &ControlGroup {
    self.values.get(name).unwrap()
  }

  pub fn get_mut(&mut self, name: &'static str) -> &mut ControlGroup {
    self.values.get_mut(name).unwrap()
  }
}

impl Index<&'static str> for Controls {
  type Output = ControlGroup;
  fn index(&self, name: &'static str) -> &Self::Output { self.get(name) }
}

impl IndexMut<&'static str> for Controls {
  fn index_mut(&mut self, name: &'static str) -> &mut Self::Output { self.get_mut(name) }
}
