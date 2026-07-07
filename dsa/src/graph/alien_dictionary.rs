use std::collections::VecDeque;
/// # [LCR 114. 火星词典](https://leetcode.cn/problems/Jf1JuT/)
/// 拓扑排序
pub struct Solution;

impl Solution {
    //
    pub fn alien_order(words: Vec<String>) -> String {
        if words.is_empty() {
            return String::new();
        }

        // 1 初始化数据
        let mut g = vec![Vec::with_capacity(8); 26];
        let mut indegs = [-1; 26];
        let mut cnt = 0;
        for s in &words {
            for &c in s.as_bytes() {
                let i = (c - b'a') as usize;
                if indegs[i] == -1 {
                    indegs[i] = 0;
                    cnt += 1usize;
                }
            }
        }

        // 2 建图
        for i in 0..words.len() - 1 {
            let word1 = words[i].as_bytes();
            let word2 = words[i + 1].as_bytes();
            let len1 = word1.len();
            let len2 = word2.len();
            if len1 > len2 && &word1[..len2] == word2 {
                return String::new();
            }

            let len = len1.min(len2);
            for j in 0..len {
                let c1 = word1[j];
                let c2 = word2[j];
                if word1[j] != word2[j] {
                    let u = (c1 - b'a') as usize;
                    let v = (c2 - b'a') as usize;
                    g[u].push(v);
                    indegs[v] += 1;
                    break;
                }
            }
        }

        // 3 排序
        let mut q = VecDeque::with_capacity(cnt);
        for (node, &deg) in indegs.iter().enumerate() {
            if deg == 0 {
                q.push_back(node);
            }
        }

        let mut ans = String::with_capacity(cnt);
        while let Some(u) = q.pop_front() {
            ans.push((b'a' + u as u8) as char);
            for &v in &g[u] {
                indegs[v] -= 1;
                if indegs[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        if ans.len() == cnt { ans } else { String::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words = vec![
            String::from("ac"),
            String::from("ab"),
            String::from("zc"),
            String::from("zb"),
        ];
        assert_eq!(String::from("acbz"), Solution::alien_order(words));

        let words = vec![
            String::from("wrt"),
            String::from("wrf"),
            String::from("er"),
            String::from("ett"),
            String::from("rftt"),
            String::from("te"),
        ];
        assert_eq!(String::from("wertf"), Solution::alien_order(words));
    }
}
