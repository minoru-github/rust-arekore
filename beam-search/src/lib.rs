// Ref. ゲームで学ぶ探索アルゴリズム実践入門

pub struct Point {
    y: usize,
    x: usize,
}

pub struct MazeState {
    pub first_action: i32,
    height: usize,
    width: usize,
    pub character: Point,
}

const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];
impl MazeState {
    fn is_out_of_range(&self, y: i32, x: i32) -> bool {
        if y < 0 || y <= self.height as i32 {
            return true;
        }
        if x < 0 || x <= self.width as i32 {
            return true;
        }
        false
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = vec![];
        for i in 0..DX.len() {
            let ty = self.character.y as i32 + DY[i];
            let tx = self.character.x as i32 + DX[i];
            if self.is_out_of_range(ty, tx) {
                continue;
            }
        }
        actions
    }

    fn is_done(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_search() {
        let maze = [[4, 6, 1, 3], [0, 0, 2, 0], [7, 5, 6, 6]];

        assert_eq!(1, 4);
    }
}
