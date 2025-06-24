use crate::robot::robot::ResourceType;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Station {
  pub x: usize,
  pub y: usize,
  pub inventory: HashMap<ResourceType, u32>,
}
