/// [17. 电话号码的字母组合](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/)
pub struct Solution;

/// rust中char类型大小固定4字节，String底层必须是utf8编码，所以这种方式往string中push char时，
/// 4 字节的 char，会重新编码（Encode）成对应的 UTF-8 字节，再塞进 String 的底层缓冲区，
/// 所以小写字母会重新编码为一个字节。对比直接处理u8会有一定性能损耗。但胜在扩展性好，可以支持复杂字符
const DICT: &[&[char]; 8] = &[
    &['a', 'b', 'c'],
    &['d', 'e', 'f'],
    &['g', 'h', 'i'],
    &['j', 'k', 'l'],
    &['m', 'n', 'o'],
    &['p', 'q', 'r', 's'],
    &['t', 'u', 'v'],
    &['w', 'x', 'y', 'z'],
];

impl Solution {
    //
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut path = String::with_capacity(digits.len());
        let mut ans = Vec::with_capacity(32);
        let digits: Vec<usize> = digits
            .as_bytes()
            .into_iter()
            .map(|&d| (d - b'2') as usize)
            .collect();
        Self::dfs(0, &digits, &mut path, &mut ans);
        ans
    }

    fn dfs(i: usize, digits: &[usize], path: &mut String, ans: &mut Vec<String>) {
        if i == digits.len() {
            ans.push(path.clone());
            return;
        }

        let digit = digits[i];
        // 枚举当前数字对应的所有字符
        for &c in DICT[digit] {
            // 固定4字节大小的char类型在底层会重新编码为utf8
            path.push(c);
            Self::dfs(i + 1, digits, path, ans);
            path.pop();
        }
    }
}

pub struct Solution2;

/// 常量定义方式： 相比方案一，此处常量最后内联到代码中时只是一个指针，但会多一次寻址，适合数据多的情况
const MAP: &[&[u8]] = &[
    b"abc", b"def", b"ghi", b"jkl", b"mno", b"pqrs", b"tuv", b"wxyz",
];

impl Solution2 {
    //
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<usize> = digits
            .as_bytes()
            .into_iter()
            .map(|&d| (d - b'2') as usize)
            .collect();

        let mut ans_cap = 1;
        for &d in &digits {
            ans_cap *= MAP[d].len();
        }
        // 避免动态扩容
        let mut ans = Vec::with_capacity(ans_cap);

        let mut path: Vec<u8> = vec![0u8; digits.len()];
        Self::dfs(0, &digits, &mut path, &mut ans);
        ans
    }

    fn dfs(i: usize, digits: &[usize], path: &mut [u8], ans: &mut Vec<String>) {
        if i == digits.len() {
            if let Ok(s) = String::from_utf8(path.to_vec()) {
                ans.push(s);
            }
            return;
        }

        let digit = digits[i];
        // 枚举当前数字对应的所有字符
        for &c in MAP[digit] {
            path[i] = c;
            Self::dfs(i + 1, digits, path, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
