// https://leetcode.com/problems/valid-sudoku/description/

use std::collections::HashSet;

fn solution(board: Vec<Vec<char>>) -> bool {
    let mut row_set = HashSet::new();
    let mut col_set = HashSet::new();
    let mut box_sets = HashSet::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(digit) = board[row][col].to_digit(10) {
                if !row_set.insert((row, digit)) {
                    return false;
                }

                if !col_set.insert((col, digit)) {
                    return false;
                }

                if !box_sets.insert(([3 * (row / 3) + col / 3], digit)) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::solutions::s0008_valid_sudoku::solution;

    #[test]
    fn it_should_return_true() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(solution(board), true);
    }
}
