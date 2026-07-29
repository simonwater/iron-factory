//! [13. 罗马数字转整数](https://leetcode.cn/problems/roman-to-integer/)
//!
//! 两种思路：
//! 1. 双字符的也只有六种，直接放到字典中和单字符平级处理，判断时优先判断双字符
//! 2. 总结规律：当前字符大于等于后一个字符，对最终结果贡献为 正；当前字符小于下一个字符，对最终结果贡献为 负。
pub struct Solution;
use std::collections::HashMap;
use std::sync::LazyLock;

// 全局静态只读 HashMap
static MAP: LazyLock<HashMap<&'static [u8], i32>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(&[b'I'] as &[u8], 1);
    m.insert(&[b'V'] as &[u8], 5);
    m.insert(&[b'X'] as &[u8], 10);
    m.insert(&[b'L'] as &[u8], 50);
    m.insert(&[b'C'] as &[u8], 100);
    m.insert(&[b'D'] as &[u8], 500);
    m.insert(&[b'M'] as &[u8], 1000);
    m.insert(b"IV" as &[u8], 4);
    m.insert(b"IX" as &[u8], 9);
    m.insert(b"XL" as &[u8], 40);
    m.insert(b"XC" as &[u8], 90);
    m.insert(b"CD" as &[u8], 400);
    m.insert(b"CM" as &[u8], 900i32);
    m
});

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut i = 0;
        while i < s.len() - 1 {
            if let Some(&val) = MAP.get(&s[i..i + 2]) {
                ans += val;
                i += 2;
            } else {
                ans += MAP.get(&s[i..i + 1]).unwrap();
                i += 1;
            }
        }
        if i < s.len() {
            ans += MAP.get(&s[i..i + 1]).unwrap();
        }
        ans
    }
}

pub struct Solution2;
const M: [i32; 22] = [
    100, 500, 0, 0, 0, 0, 1, 0, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
];
impl Solution2 {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        for i in 0..s.len() {
            let cur = M[(s[i] - b'C') as usize];
            if i < s.len() - 1 && cur < M[(s[i + 1] - b'C') as usize] {
                ans -= cur;
            } else {
                ans += cur;
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
