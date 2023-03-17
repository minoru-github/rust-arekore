#![allow(unused)]

const END_TURN: usize = 4;
const DX: [usize; 4] = [1, 0, !0, 0];
const DY: [usize; 4] = [0, 1, 0, !0];
const HEIGHT: usize = 3;
const WIDTH: usize = 4;

#[derive(Clone)]
pub struct Pos {
    pub y: usize,
    pub x: usize,
}

impl Pos {
    pub fn new(y: usize, x: usize) -> Self {
        Pos { y, x }
    }
}

#[derive(Clone)]
pub struct State {
    pub first_action: Option<usize>,
    pub character: Pos,
    pub points: Vec<Vec<usize>>,
    pub turn: usize,
    pub game_score: usize,
    pub evaluated_score: usize,
}

impl State {
    pub fn new(character: Pos, mut points: Vec<Vec<usize>>) -> Self {
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

    pub fn advance(&mut self, action: usize) {
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

    pub fn legal_actions(&self) -> Vec<usize> {
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

    pub fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    pub fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }

    pub fn debug(&self) {
        println!("# turn {}", self.turn);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.character.y == y && self.character.x == x {
                    print!("@ ");
                } else {
                    print!("{} ", self.points[y][x]);
                }
            }
            println!();
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
}