#![allow(unused)]
use std::cmp::Ordering;

pub struct Node {
    pub id: usize,
    parent: Option<usize>,
    childs: Vec<usize>,
    pub score: i32,
}

impl Node {
    pub fn new(id: usize, score: i32) -> Self {
        Node {
            id,
            parent: None,
            childs: vec![],
            score,
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}