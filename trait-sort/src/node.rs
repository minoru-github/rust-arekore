#![allow(unused)]
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
