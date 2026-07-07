/// [130. 被围绕的区域](https://leetcode.cn/problems/surrounded-regions/)
pub struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        for c in 0..n {
            Self::dfs(board, 0, c as i32);
            Self::dfs(board, m as i32 - 1, c as i32);
        }
        for r in 0..m {
            Self::dfs(board, r as i32, 0);
            Self::dfs(board, r as i32, n as i32 - 1);
        }
        for row in board {
            for val in row {
                if *val == 'O' {
                    *val = 'X';
                } else if *val == '*' {
                    *val = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, r: i32, c: i32) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        if r < 0 || r == m || c < 0 || c == n {
            return;
        }
        let r_idx = r as usize;
        let c_idx = c as usize;
        if board[r_idx][c_idx] == 'O' {
            board[r_idx][c_idx] = '*';
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                Self::dfs(board, r + dr, c + dc);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
