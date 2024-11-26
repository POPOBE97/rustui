use std::collections::BTreeMap;

#[derive(Debug)]
pub struct PersistOrderMap<K, V> {
  pub inner: BTreeMap<K, V>,
  pub order: Vec<K>,
}

pub struct PersistOrderMapIterator<'a, K, V> {
  map: &'a PersistOrderMap<K, V>,
  index: usize,
}

pub struct IntoIter<K, V> {
  map: PersistOrderMap<K, V>,
  index: usize,
}

impl<K, V> PersistOrderMap<K, V>
where
  K: Ord + Copy,
{
  pub fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
      order: Vec::new(),
    }
  }

  pub fn insert(&mut self, key: K, value: V) {
    if !self.order.contains(&key) {
      self.order.push(key);
    }
    self.inner.insert(key, value);
  }

  pub fn iter(&self) -> PersistOrderMapIterator<'_, K, V> {
    PersistOrderMapIterator {
      map: self,
      index: 0,
    }
  }

  pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
    self.inner.get_mut(&key)
  }

  pub fn get(&self, key: K) -> Option<&V> {
    self.inner.get(&key)
  }
}

impl<'a, K, V> Iterator for PersistOrderMapIterator<'a, K, V>
where
  K: Ord + Copy,
{
  type Item = (&'a K, &'a V);

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.map.order.len() {
      let key = &self.map.order[self.index];
      self.index += 1;
      self.map.inner.get_key_value(key)
    } else {
      None
    }
  }
}

impl<K, V> Iterator for IntoIter<K, V>
where
  K: Ord + Copy,
{
  type Item = (K, V);

  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.map.order.len() {
      let key = self.map.order[self.index];
      self.index += 1;
      self.map.inner.remove(&key).map(|value| (key, value))
    } else {
      None
    }
  }
}

impl<K, V> IntoIterator for PersistOrderMap<K, V>
where
  K: Ord + Copy,
{
  type Item = (K, V);
  type IntoIter = IntoIter<K, V>;

  fn into_iter(self) -> Self::IntoIter {
    IntoIter {
      map: self,
      index: 0,
    }
  }
}

impl<'a, K, V> IntoIterator for &'a PersistOrderMap<K, V>
where
  K: Ord + Copy,
{
  type Item = (&'a K, &'a V);
  type IntoIter = PersistOrderMapIterator<'a, K, V>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}