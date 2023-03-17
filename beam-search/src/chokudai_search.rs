#![allow(unused)]
use crate::state::*;
// Ref. ゲームで学ぶ探索アルゴリズム実践入門

use std::collections::BinaryHeap;

fn chokudai_search(state: &State, beam_width: usize, beam_depth: usize, beam_number:usize) -> Option<usize>{
    let mut beam = vec![];
    for t in 0..beam_width+1 {
        beam.push(BinaryHeap::new());
    }
    beam[0].push(state.clone());
    for cnt in 0..beam_number {
        for t in 0..beam_depth {
            //let mut now_beam = &mut beam[t];
            //let mut next_beam = &mut beam[t+1];
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
                    beam[t+1].push(next_state);
                }
            }
        }
    }

    for now_beam in beam {
        if !now_beam.is_empty() {
            return now_beam.peek().unwrap().first_action;
        }
    }
    None
}
