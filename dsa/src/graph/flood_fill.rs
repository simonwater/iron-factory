/// [733. 图像渲染](https://leetcode.cn/problems/flood-fill/)
pub struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let old_color = image[sr as usize][sc as usize];
        if old_color != color {
            Self::dfs(&mut image, sr, sc, old_color, color);
        }
        image
    }

    fn dfs(image: &mut Vec<Vec<i32>>, r: i32, c: i32, old_color: i32, new_color: i32) {
        let m = image.len() as i32;
        let n = image[0].len() as i32;
        if r < 0 || r == m || c < 0 || c == n || image[r as usize][c as usize] != old_color {
            return;
        }
        image[r as usize][c as usize] = new_color;
        Self::dfs(image, r + 1, c, old_color, new_color);
        Self::dfs(image, r - 1, c, old_color, new_color);
        Self::dfs(image, r, c + 1, old_color, new_color);
        Self::dfs(image, r, c - 1, old_color, new_color);
    }
}

pub struct Solution2;
use std::collections::VecDeque;
impl Solution2 {
    /// bfs
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let old_color = image[sr as usize][sc as usize];
        if old_color == color {
            return image;
        }
        // 先变色再入队列。如果出队列的时候才变色，那么同一个节点会被多个邻居扫描到而重复入队列。
        image[sr as usize][sc as usize] = color;
        let mut q = VecDeque::new();
        q.push_back((sr, sc));
        let m = image.len() as i32;
        let n = image[0].len() as i32;
        while let Some((r, c)) = q.pop_front() {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_r = r + dr;
                let new_c = c + dc;
                if new_r >= 0
                    && new_r < m
                    && new_c >= 0
                    && new_c < n
                    && image[new_r as usize][new_c as usize] == old_color
                {
                    image[new_r as usize][new_c as usize] = color; // 先改变颜色再入队列
                    q.push_back((new_r, new_c));
                }
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let color = 2;
        let output = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(Solution::flood_fill(image, sr, sc, color), output);
    }

    #[test]
    fn test2() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr = 1;
        let sc = 1;
        let color = 2;
        let output = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(Solution2::flood_fill(image, sr, sc, color), output);
    }
}
