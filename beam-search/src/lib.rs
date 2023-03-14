#![allow(unused)]
// Ref. ゲームで学ぶ探索アルゴリズム実践入門

const END_TURN: usize = 4;
const DX: [usize; 4] = [1, 0, !0, 0];
const DY: [usize; 4] = [0, 1, 0, !0];
const HEIGHT: usize = 4;
const WIDTH: usize = 4;

pub struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(y: usize, x: usize) -> Self {
        Pos { y, x }
    }
}

pub struct MazeState {
    first_action: i32,
    character: Pos,
    points: Vec<Vec<usize>>,
    turn: usize,
    game_score: usize,
}

impl MazeState {
    fn new(character: Pos, mut points: Vec<Vec<usize>>) -> Self {
        points[character.y][character.x] = 0;
        MazeState {
            first_action: -1,
            character,
            points,
            turn: 0,
            game_score: 0,
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

    fn is_done(&self) -> bool {
        self.turn >= END_TURN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut maze_state = MazeState::new(character, points);
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
        let mut maze_state = MazeState::new(character, points);
        let legal_actions = maze_state.legal_actions();
        let expect = vec![0, 1];
        assert_eq!(legal_actions, expect);

        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut maze_state = MazeState::new(character, points);
        let legal_actions = maze_state.legal_actions();
        let expect = vec![0, 1, 2, 3];
        assert_eq!(legal_actions, expect);
    }

    #[test]
    fn test_beam_search() {
        let character = Pos::new(1, 1);
        let points = vec![vec![4, 6, 1, 3], vec![0, 0, 2, 0], vec![7, 5, 6, 6]];
        let mut maze_state = MazeState::new(character, points);

        assert_eq!(1, 1);
    }
}
