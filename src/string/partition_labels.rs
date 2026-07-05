/// # [763. 划分字母区间](https://leetcode.cn/problems/partition-labels/)
///

pub struct Solution;

impl Solution {
    //
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s_bytes = s.as_bytes();
        let mut last_pos = [-1; 26];
        for (i, &c) in s_bytes.iter().enumerate() {
            last_pos[(c - b'a') as usize] = i as i32;
        }
        let mut ans = Vec::with_capacity(s.len());
        let (mut start, mut end) = (0, 0);
        for (i, &c) in s_bytes.iter().enumerate() {
            end = end.max(last_pos[(c - b'a') as usize] as i32);
            if i as i32 == end {
                ans.push(end - start + 1);
                start = end + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
