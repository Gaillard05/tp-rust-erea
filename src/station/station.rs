use std::collections::HashMap;
use crate::robot::robot::ResourceType;

#[derive(Debug)]
pub struct Station {
    pub x: usize,
    pub y: usize,
    pub inventory: HashMap<ResourceType, u32>,
}