//! [953. 验证外星语词典](https://leetcode.cn/problems/verifying-an-alien-dictionary/)
//!

pub struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut order_map = [0; 26];
        for (i, &c) in order.as_bytes().iter().enumerate() {
            order_map[(c - b'a') as usize] = i as u8;
        }

        for i in 0..words.len() - 1 {
            if !Self::compare(words[i].as_bytes(), words[i + 1].as_bytes(), &order_map) {
                return false;
            }
        }

        true
    }

    fn compare(w1: &[u8], w2: &[u8], order: &[u8]) -> bool {
        let len = w1.len().min(w2.len());
        for i in 0..len {
            let c1 = (w1[i] - b'a') as usize;
            let c2 = (w2[i] - b'a') as usize;
            if order[c1] != order[c2] {
                return order[c1] < order[c2];
            }
        }
        w1.len() <= w2.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
