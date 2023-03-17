#![allow(unused)]
use crate::state::*;
// Ref. ゲームで学ぶ探索アルゴリズム実践入門

use std::collections::BinaryHeap;

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
        let state = State::new(character, points);
        let legal_actions = state.legal_actions();
        let expect = vec![0, 1];
        assert_eq!(legal_actions, expect);

        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let legal_actions = state.legal_actions();
        let expect = vec![0, 1, 2, 3];
        assert_eq!(legal_actions, expect);
    }

    #[test]
    fn test_beam_search_w2_d4() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let beam_width = 2;
        let beam_depth = 4;
        let action = beam_search(&state, beam_width, beam_depth);
        assert_eq!(action, Some(1));
    }

        #[test]
    fn test_beam_search_w4_d4() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let beam_width = 4;
        let beam_depth = 4;
        let action = beam_search(&state, beam_width, beam_depth);
        assert_eq!(action, Some(0));
    }

    #[test]
    fn test_beam_search_w4_d1() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let beam_width = 4;
        let beam_depth = 1;
        let action = beam_search(&state, beam_width, beam_depth);
        assert_eq!(action, Some(3));
    }
}
