#![allow(unused)]
use std::cmp::Ordering;
use crate::Node;

// https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html#how-can-i-implement-ord
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Node {}
// or #[derive(Eq)] to struct

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn test_sort_by_one_comparison() {
        let nodes = [
            Node::new(0, 10),
            Node::new(1, -20),
            Node::new(2, 100),
            Node::new(3, 100),
        ];

        let mut q = BinaryHeap::new();
        for node in nodes {
            q.push(node);
        }

        let mut ans = vec![];
        while !q.is_empty() {
            let node = q.pop().unwrap();
            //println!("{} {}", node.id, node.score);
            ans.push(node.id);
        }

        let est = vec![2,3,0,1];
        assert_eq!(ans,est);
    }
}
