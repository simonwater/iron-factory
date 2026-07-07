/// # [433. 最小基因变化](https://leetcode.cn/problems/minimum-genetic-mutation/)
///
/// 所有权流转 + 零拷贝 + 就地take
pub struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    //
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        if start_gene == end_gene {
            return 0;
        }
        let mut set: HashSet<String> = HashSet::from_iter(bank);
        if !set.contains(&end_gene) {
            return -1;
        }
        let mut q = VecDeque::with_capacity(set.len());
        set.remove(&start_gene);
        q.push_back((start_gene, 0));
        while let Some((node, step)) = q.pop_front() {
            let mut cur_bytes = node.into_bytes();
            for i in 0..cur_bytes.len() {
                let old = cur_bytes[i];
                for g in [b'A', b'C', b'G', b'T'] {
                    if g != old {
                        cur_bytes[i] = g;
                        let new_str = std::str::from_utf8(&cur_bytes).unwrap();
                        if let Some(next) = set.take(new_str) {
                            if next == end_gene {
                                return step + 1;
                            }
                            q.push_back((next, step + 1));
                        }
                    }
                }
                cur_bytes[i] = old;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
