/// # [6. Z 字形变换](https://leetcode.cn/problems/zigzag-conversion/)
///

/**
 *
"PINALSIGYAHRPI"  ->  "PINALSIGYAHRPI"
P     I    N
A   L S  I G
Y A   H R
P     I
 *
 */

pub struct Solution;

impl Solution {
    //
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= 2 {
            return s;
        }
        let rows = num_rows as usize;
        let mut ans: Vec<Vec<u8>> = vec![Vec::with_capacity(16); rows];
        let bytes = s.as_bytes();
        let mut r = 0;
        let mut direction = -1;
        for &c in bytes {
            ans[r as usize].push(c);
            if r == 0 || r == num_rows - 1 {
                direction = -direction;
            }
            r += direction;
        }

        String::from_utf8(ans.concat()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
