/// [417. 太平洋大西洋水流问题](https://leetcode.cn/problems/pacific-atlantic-water-flow/)
struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut lefts_tops = vec![vec![false; n]; m];
        let mut rights_bottoms = vec![vec![false; n]; m];
        // 染色
        let m_i32 = m as i32;
        let n_i32 = n as i32;
        for r in 0..m {
            Self::dfs(r as i32, 0, 0, &heights, &mut lefts_tops);
            Self::dfs(r as i32, n_i32 - 1, 0, &heights, &mut rights_bottoms);
        }
        for c in 0..n {
            Self::dfs(0, c as i32, 0, &heights, &mut lefts_tops);
            Self::dfs(m_i32 - 1, c as i32, 0, &heights, &mut rights_bottoms);
        }

        let mut ans = Vec::with_capacity(16);
        for i in 0..m {
            for j in 0..n {
                if lefts_tops[i][j] && rights_bottoms[i][j] {
                    ans.push(vec![i as i32, j as i32])
                }
            }
        }
        ans
    }

    fn dfs(r: i32, c: i32, prev: i32, heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) {
        let m = heights.len() as i32;
        let n = heights[0].len() as i32;
        if r < 0 || r >= m || c < 0 || c >= n {
            return;
        }
        let r_idx = r as usize;
        let c_idx = c as usize;
        let cur = heights[r_idx][c_idx];
        if visited[r_idx][c_idx] || prev > cur {
            return;
        }
        visited[r_idx][c_idx] = true;
        Self::dfs(r + 1, c, cur, heights, visited);
        Self::dfs(r - 1, c, cur, heights, visited);
        Self::dfs(r, c + 1, cur, heights, visited);
        Self::dfs(r, c - 1, cur, heights, visited);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
