/// [79. 单词搜索](https://leetcode.cn/problems/word-search/)
pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut word_bytes = word.into_bytes();
        let m = board.len();
        let n = board[0].len();
        let mut counter = [0; 60];

        // 优化1: 统计字符数量，对于同一个字符，如果board中的数量还比word中少，则肯定不存在
        for row in &board {
            for &val in row {
                counter[(val as u8 - b'A') as usize] += 1;
            }
        }
        let start_cnt = counter[(word_bytes[0] - b'A') as usize];
        let end_cnt = counter[(word_bytes[word_bytes.len() - 1] - b'A') as usize];
        for &c in &word_bytes {
            let idx = (c - b'A') as usize;
            if counter[idx] == 0 {
                // 已经不够消耗
                return false;
            }
            counter[idx] -= 1;
        }

        // 优化2: board中开头字符数量多于结尾数量，则反转字符串再进行dfs
        if start_cnt > end_cnt {
            word_bytes.reverse();
        }

        let mut visited = vec![vec![false; n]; m];
        for (r, row) in board.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val as u8 == word_bytes[0]
                    && Self::dfs(0, &word_bytes, r as i32, c as i32, &board, &mut visited)
                {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        i: usize,
        word_bytes: &[u8],
        r: i32,
        c: i32,
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if i == word_bytes.len() {
            return true;
        }
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        if r < 0 || r >= m || c < 0 || c >= n {
            return false;
        }
        let r_idx = r as usize;
        let c_idx = c as usize;
        if visited[r_idx][c_idx] || board[r_idx][c_idx] != word_bytes[i] as char {
            return false;
        }
        visited[r_idx][c_idx] = true;
        let ans = Self::dfs(i + 1, word_bytes, r + 1, c, board, visited)
            || Self::dfs(i + 1, word_bytes, r - 1, c, board, visited)
            || Self::dfs(i + 1, word_bytes, r, c + 1, board, visited)
            || Self::dfs(i + 1, word_bytes, r, c - 1, board, visited);

        // 回溯至上一层前把当前层恢复到旧现场
        visited[r_idx][c_idx] = false;
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
