//! [36. 有效的数独](https://leetcode.cn/problems/valid-sudoku/)
//! > 坐标转换

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![vec![false; 9]; 9];
        let mut cols = vec![vec![false; 9]; 9];
        let mut cells = vec![vec![false; 9]; 9];
        for (r, row) in board.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == '.' {
                    continue;
                }
                let cell_idx = (r / 3) * 3 + c / 3;
                let val_idx = ((ch as u8) - b'1') as usize;
                if rows[r][val_idx] || cols[c][val_idx] || cells[cell_idx][val_idx] {
                    return false;
                }
                rows[r][val_idx] = true;
                cols[c][val_idx] = true;
                cells[cell_idx][val_idx] = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
