//! A context is a special edge that gives weight to zero or more options. If weight and tag are not
//! given, it will return a semi-random value.

use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

#[derive(Clone, Debug)]
struct ByWeight<T>
where
  T: Clone,
{
  pub lookup: HashMap<String, Arc<RwLock<T>>>,
}

impl<T> ByWeight<T>
where
  T: Clone,
{
  pub fn new() -> ByWeight<T> {
    ByWeight {
      lookup: HashMap::new(),
    }
  }
}

#[derive(Clone, Debug)]
struct ByTag<T> {
  pub lookup: HashMap<u32, Arc<RwLock<T>>>,
}

impl<T> ByTag<T> {
  pub fn new() -> ByTag<T> {
    ByTag {
      lookup: HashMap::new(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct ContextEdge<T>
where
  T: Clone,
{
  by_weight: HashMap<u32, ByWeight<T>>,
  by_tag: HashMap<String, ByTag<T>>,
}

impl<T> ContextEdge<T>
where
  T: Clone,
{
  pub fn new() -> ContextEdge<T> {
    ContextEdge {
      by_weight: HashMap::new(),
      by_tag: HashMap::new(),
    }
  }

  pub fn add(
    &mut self,
    item: Arc<RwLock<T>>,
    tag: String,
    weight: u32,
  ) -> Result<Option<Arc<RwLock<T>>>, String> {
    let weight_result = match self.by_weight.get(&weight) {
      Some(values) => {
        let mut x: ByWeight<T> = values.clone();
        let result = x.lookup.insert(tag.clone(), item.clone());
        self.by_weight.insert(weight, x);
        result
      }
      None => {
        let mut lookup = ByWeight::new();
        lookup.lookup.insert(tag.clone(), item.clone());
        self.by_weight.insert(weight, lookup);
        None
      }
    };
    let tag_result = match self.by_tag.get(&tag) {
      Some(values) => {
        let mut x: ByTag<T> = values.clone();
        let result = x.lookup.insert(weight, item);
        self.by_tag.insert(tag, x);
        result
      }
      None => {
        let mut lookup = ByTag::new();
        lookup.lookup.insert(weight, item);
        self.by_tag.insert(tag, lookup);
        None
      }
    };

    match (&tag_result, &weight_result) {
      (Some(inner_tag), Some(inner_weight)) => {
        if Arc::ptr_eq(&inner_tag, &inner_weight) {
          Ok(tag_result)
        } else {
          panic!("Tag edge does not equal weight edge. Exiting");
        }
      }
      (None, None) => Ok(None),
      _ => panic!("Tag edge does not equal weight edge. Exiting"),
    }
  }

  pub fn try_heaviest(&self, tag: Option<String>) -> Option<Arc<RwLock<T>>> {
    match self.by_weight.keys().max() {
      Some(max) => {
        let values = self.by_weight.get(max).unwrap();
        match tag {
          Some(tag) => values.lookup.get(&tag).map(|value| value.clone()),
          None => values.lookup.values().last().map(|value| value.clone()),
        }
      }
      None => None,
    }
  }

  pub fn try_tag(&self, tag: String) -> Option<Arc<RwLock<T>>> {
    match self.by_tag.get(&tag) {
      Some(by_tag) => match by_tag.lookup.keys().max() {
        Some(max) => by_tag.lookup.get(max).map(|value| value.clone()),
        None => None,
      },
      None => None,
    }
  }
}
