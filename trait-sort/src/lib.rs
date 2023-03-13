#![allow(unused)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Node {
    id: usize,
    parent: Option<usize>,
    childs: Vec<usize>,
    score: i32,
}

impl Node {
    fn new(id: usize, score: i32) -> Self {
        Node {
            id,
            parent: None,
            childs: vec![],
            score,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sort() {
        let nodes = [
             Node::new(0, 10),
             Node::new(1, -20),
             Node::new(2, 100)
        ];

        let mut q = BinaryHeap::new();
        for node in nodes {
            q.push(node);
        }
    }
}
