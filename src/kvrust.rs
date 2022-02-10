use std::collections::HashMap;
use std::hash;

pub struct Kvrust<K, V> {
  pub store: HashMap<K, V>,
}

impl<K: Clone + Eq + hash::Hash, V: Default + Copy> Kvrust<K, V> {
  pub fn new() -> Kvrust<K, V> {
    let store: HashMap<K, V> = HashMap::new();
    Kvrust { store }
  }

  pub fn add(&mut self, k: K, v: V) {
    self.store.insert(k, v);
  }

  pub fn get(&self, k: K) -> V {
    match self.store.get(&k).cloned() {
      None => Default::default(),
      Some(v) => v,
    }
  }
}
