pub mod solver {
    pub fn valid(board: [[i8;9]; 9], row: usize, column: usize, guess: i8) -> bool {
        for x in 0..9 {
            if board[row][x] == guess || board[x][column] == guess {
                return false
            }
        }
        let x_index: usize = row / 3 * 3;
        let y_index: usize = column / 3 * 3;

        for x in 0..3 {
            for y in 0..3 {
                if board[x_index + x][y_index + y] == guess {
                    return false
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::solver;

    fn test_board() -> [[i8;9];9] {
        [
            [1,7,4,0,9,0,6,0,0],
            [0,0,0,0,3,8,1,5,7],
            [5,3,0,7,0,1,0,0,4],
            [0,0,7,3,4,9,8,0,0],
            [8,4,0,5,0,0,3,6,0],
            [3,0,5,0,0,6,4,7,0],
            [2,8,6,9,0,0,0,0,1],
            [0,0,0,6,2,7,0,3,8],
            [0,5,3,0,8,0,0,9,6]
        ]
    }

    #[test]
    fn returns_invalid_for_1_0_4() {
        assert!(!solver::valid(test_board(), 1, 0, 4))
    }

    #[test]
    fn returns_invalid_for_1_0_1() {
        assert!(!solver::valid(test_board(), 1, 0, 1))
    }

    #[test]
    fn returns_invalid_for_1_1_4() {
        assert!(!solver::valid(test_board(), 1, 1, 4))
    }

    #[test]
    fn returns_invalid_for_1_2_4() {
        assert!(!solver::valid(test_board(), 1, 2, 4))
    }

    #[test]
    fn returns_invalid_for_0_3_4() {
        assert!(!solver::valid(test_board(), 0, 3, 4))
    }

    #[test]
    fn returns_invalid_for_0_3_3() {
        assert!(!solver::valid(test_board(), 0 , 3, 3))
    }

    #[test]
    fn returns_invalid_for_6_6_3() {
        assert!(!solver::valid(test_board(), 6, 6, 3))
    }

    #[test]
    fn returns_invalid_for_8_6_1() {
        assert!(!solver::valid(test_board(), 8, 6, 1))
    }

    #[test]
    fn returns_valid_for_0_3_2() {
        assert!(solver::valid(test_board(), 0, 3, 2))
    }
}
