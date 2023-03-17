#![allow(unused)]
use crate::state::*;
use std::cell::RefCell;
use std::rc::Rc;
// Ref. ゲームで学ぶ探索アルゴリズム実践入門

use std::collections::BinaryHeap;

fn chokudai_search(
    state: &State,
    beam_width: usize,
    beam_depth: usize,
    beam_number: usize,
) -> Option<usize> {
    let mut beam = vec![];
    for t in 0..beam_depth + 1 {
        beam.push(BinaryHeap::new());
    }
    beam[0].push(state.clone());
    for cnt in 0..beam_number {
        for t in 0..beam_depth {
            for i in 0..beam_width {
                if beam[t].is_empty() {
                    break;
                }
                let now_state = beam[t].pop().unwrap();
                if now_state.is_done() {
                    break;
                }

                let legal_actions = now_state.legal_actions();
                for action in legal_actions {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    if t == 0 {
                        next_state.first_action = Some(action);
                    }
                    beam[t + 1].push(next_state);
                }
            }
        }
    }

    // for state in beam[beam_depth].iter() {
    //     state.debug();
    // }

    for now_beam in beam.iter().rev() {
        if !now_beam.is_empty() {
            return now_beam.peek().unwrap().first_action;
        }
    }
    None
}

// RefCell遅いので、参考までに
fn chokudai_search_with_refcell(
    state: &State,
    beam_width: usize,
    beam_depth: usize,
    beam_number: usize,
) -> Option<usize> {
    let mut beam = vec![];
    for t in 0..beam_depth + 1 {
        beam.push(Rc::new(RefCell::new(BinaryHeap::new())));
    }
    beam[0].borrow_mut().push(state.clone());
    for cnt in 0..beam_number {
        for t in 0..beam_depth {
            let now_beam = beam[t].clone();
            let next_beam = beam[t + 1].clone();
            for i in 0..beam_width {
                if now_beam.borrow().is_empty() {
                    break;
                }
                let now_state = now_beam.borrow_mut().pop().unwrap();
                if now_state.is_done() {
                    break;
                }

                let legal_actions = now_state.legal_actions();
                for action in legal_actions {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    if t == 0 {
                        next_state.first_action = Some(action);
                    }
                    next_beam.borrow_mut().push(next_state);
                }
            }
        }
    }

    // for state in beam[beam_depth].iter() {
    //     state.debug();
    // }

    for now_beam in beam.iter().rev() {
        if !now_beam.borrow().is_empty() {
            return now_beam.borrow().peek().unwrap().first_action;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_search_w1_d1_n4() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let beam_width = 1;
        let beam_depth = 1;
        let beam_number = 4;
        let action = chokudai_search(&state, beam_width, beam_depth, beam_number);
        assert_eq!(action, Some(3));
    }

    #[test]
    fn test_beam_search_w1_d2_n2() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let beam_width = 1;
        let beam_depth = 2;
        let beam_number = 2;
        let action = chokudai_search(&state, beam_width, beam_depth, beam_number);
        assert_eq!(action, Some(1));
    }

    #[test]
    fn test_beam_search_w1_d3_n2() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let beam_width = 1;
        let beam_depth = 3;
        let beam_number = 2;
        let action = chokudai_search(&state, beam_width, beam_depth, beam_number);
        assert_eq!(action, Some(1));
    }

    #[test]
    fn test_beam_search_w1_d4_n4() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let state = State::new(character, points);
        let beam_width = 1;
        let beam_depth = 4;
        let beam_number = 2;
        let action = chokudai_search(&state, beam_width, beam_depth, beam_number);
        assert_eq!(action, Some(1));
    }
}
