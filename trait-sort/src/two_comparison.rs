#![allow(unused)]
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Point {
    pub id: usize,
    pub dist: usize,
    pub score: usize,
}

impl Point {
    pub fn new(id: usize, dist: usize, score: usize) -> Self {
        Point { id, dist, score }
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.dist == other.dist) && (self.score == other.score)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 最短経路探索等で距離が近いほうを優先的にBinaryHeap(優先度付きキュー)からpop()したいので
        // distが近いほうをOrdering::Greaterに設定する
        if self.dist < other.dist {
            Some(Ordering::Greater)
        } else if self.dist > other.dist {
            Some(Ordering::Less)
        } else {
            // 仮実装。distが同じならscoreがより高いほうをpop()。
            if self.score > other.score {
                Some(Ordering::Greater)
            } else if self.score < other.score {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // 最短経路探索等で距離が近いほうを優先的にBinaryHeap(優先度付きキュー)からpop()したいので
        // distが近いほうをOrdering::Greaterに設定する
        if self.dist < other.dist {
            Ordering::Greater
        } else if self.dist > other.dist {
            Ordering::Less
        } else {
            // 仮実装。distが同じならscoreがより高いほうをpop()。
            if self.score > other.score {
                Ordering::Greater
            } else if self.score < other.score {
                Ordering::Less
            } else {
                // distもscoreも同じ場合、BinaryHeap(優先度付きキュー)だとidが大きいものが優先的にpopされる
                // BinaryHeap(優先度付きキュー)にpush()した順ではないので注意。
                Ordering::Equal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;

    #[test]
    fn test_two_comparison() {
        // 今回の場合、distとscoreを使ったOrdしか実装していない。
        // distもscoreも同じなら構造体の上位メンバーを比較して大きいものが優先的にpop()される。
        // BinaryHeap(優先度付きキュー)にpush()した順ではないので注意。
        let points = [
            Point::new(0, 2, 10),
            Point::new(1, 0, 30),
            Point::new(2, 2, 10),
            Point::new(3, 30, 0),
            Point::new(4, 5, 100),
        ];

        let mut q = BinaryHeap::from(points);
        let mut ans = vec![];
        while !q.is_empty() {
            let point = q.pop().unwrap();
            ans.push(point.id);
        }

        let est = vec![1, 2, 0, 4, 3];
        assert_eq!(ans, est);
    }
}
