//! [37. 解数独](https://leetcode.cn/problems/sudoku-solver)
//!

pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxs = [[false; 9]; 9];
        let mut spaces = Vec::with_capacity(81);
        for (r, row) in board.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == '.' {
                    spaces.push((r, c));
                } else {
                    let val_idx = ((ch as u8) - b'1') as usize;
                    let box_idx = (r / 3) * 3 + c / 3;
                    rows[r][val_idx] = true;
                    cols[c][val_idx] = true;
                    boxs[box_idx][val_idx] = true;
                }
            }
        }

        Self::dfs(0, board, &spaces, &mut rows, &mut cols, &mut boxs);
    }

    fn dfs(
        i: usize,
        board: &mut Vec<Vec<char>>,
        spaces: &[(usize, usize)],
        rows: &mut [[bool; 9]; 9],
        cols: &mut [[bool; 9]; 9],
        boxs: &mut [[bool; 9]; 9],
    ) -> bool {
        if i >= spaces.len() {
            return true;
        }
        let (r, c) = spaces[i];
        let box_idx = (r / 3) * 3 + c / 3;
        // 在当前空格枚举所有合法的数字进行填充
        for val in '1'..='9' {
            let val_idx = ((val as u8) - b'1') as usize;
            // 确定合法性
            if !rows[r][val_idx] && !cols[c][val_idx] && !boxs[box_idx][val_idx] {
                rows[r][val_idx] = true;
                cols[c][val_idx] = true;
                boxs[box_idx][val_idx] = true;
                board[r][c] = val;
                // 继续下一格
                if Self::dfs(i + 1, board, spaces, rows, cols, boxs) {
                    return true;
                } else {
                    // 当前选择不成功，恢复状态，尝试下一个数字
                    rows[r][val_idx] = false;
                    cols[c][val_idx] = false;
                    boxs[box_idx][val_idx] = false;
                    board[r][c] = '.';
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
