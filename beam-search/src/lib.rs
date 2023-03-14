#![allow(unused)]
// Ref. ゲームで学ぶ探索アルゴリズム実践入門

use std::collections::BinaryHeap;

const END_TURN: usize = 4;
const DX: [usize; 4] = [1, 0, !0, 0];
const DY: [usize; 4] = [0, 1, 0, !0];
const HEIGHT: usize = 3;
const WIDTH: usize = 4;

#[derive(Clone)]
pub struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(y: usize, x: usize) -> Self {
        Pos { y, x }
    }
}

#[derive(Clone)]
pub struct State {
    first_action: Option<usize>,
    character: Pos,
    points: Vec<Vec<usize>>,
    turn: usize,
    game_score: usize,
    evaluated_score: usize,
}

impl State {
    fn new(character: Pos, mut points: Vec<Vec<usize>>) -> Self {
        points[character.y][character.x] = 0;
        State {
            first_action: None,
            character,
            points,
            turn: 0,
            game_score: 0,
            evaluated_score: 0,
        }
    }

    fn advance(&mut self, action: usize) {
        let ty = self.character.y.wrapping_add(DY[action]);
        let tx = self.character.x.wrapping_add(DX[action]);

        self.character.y = ty;
        self.character.x = tx;
        let point = self.points[ty][tx];
        if point > 0 {
            self.game_score += point;
            self.points[ty][tx] = 0;
        }
        self.turn += 1;
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = vec![];
        for action in 0..DX.len() {
            let ty = self.character.y.wrapping_add(DY[action]);
            let tx = self.character.x.wrapping_add(DX[action]);

            if HEIGHT <= ty || WIDTH <= tx {
                continue;
            }
            actions.push(action);
        }
        actions
    }

    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }

    fn debug(&self) {
        println!("# turn {}", self.turn);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.character.y == y && self.character.x == x {
                    print!("@ ");
                } else {
                    print!("{} ", self.points[y][x]);
                }
            }
            println!("");
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.evaluated_score == other.evaluated_score
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.evaluated_score.cmp(&other.evaluated_score))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}

fn beam_search(state: &State, beam_width: usize, beam_depth: usize) -> Option<usize> {
    let mut now_beam = BinaryHeap::new();
    let initial_state = state.clone();
    let mut best_state = initial_state.clone();

    now_beam.push(initial_state);
    for t in 0..beam_depth {
        let mut next_beam = BinaryHeap::new();
        for i in 0..beam_width {
            if now_beam.is_empty() {
                break;
            }
            let now_state = now_beam.pop().unwrap();
            let legal_actions = now_state.legal_actions();
            for action in legal_actions {
                let mut next_state = now_state.clone();
                next_state.advance(action);
                next_state.evaluate_score();
                if t == 0 {
                    next_state.first_action = Some(action);
                }
                next_beam.push(next_state);
            }
        }

        now_beam = next_beam;
        best_state = now_beam.peek().unwrap().clone();
        best_state.debug();
        if best_state.is_done() {
            break;
        }
    }
    best_state.first_action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut maze_state = State::new(character, points);
        let action = 0;
        maze_state.advance(action);
        let action = 1;
        maze_state.advance(action);

        assert_eq!(maze_state.turn, 2);
        assert_eq!(maze_state.character.y, 2);
        assert_eq!(maze_state.character.x, 2);
        assert_eq!(maze_state.game_score, 8);
    }

    #[test]
    fn test_legal_actions() {
        let character = Pos::new(0, 0);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut state = State::new(character, points);
        let legal_actions = state.legal_actions();
        let expect = vec![0, 1];
        assert_eq!(legal_actions, expect);

        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut state = State::new(character, points);
        let legal_actions = state.legal_actions();
        let expect = vec![0, 1, 2, 3];
        assert_eq!(legal_actions, expect);
    }

    #[test]
    fn test_beam_search_w2_d4() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut state = State::new(character, points);
        let beam_width = 2;
        let beam_depth = 4;
        let action = beam_search(&state, beam_width, beam_depth);
        assert_eq!(action, Some(1));
    }

        #[test]
    fn test_beam_search_w4_d4() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut state = State::new(character, points);
        let beam_width = 4;
        let beam_depth = 4;
        let action = beam_search(&state, beam_width, beam_depth);
        assert_eq!(action, Some(0));
    }

    #[test]
    fn test_beam_search_w4_d1() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut state = State::new(character, points);
        let beam_width = 4;
        let beam_depth = 1;
        let action = beam_search(&state, beam_width, beam_depth);
        assert_eq!(action, Some(3));
    }
}
