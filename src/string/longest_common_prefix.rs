/// [14. 最长公共前缀](https://leetcode.cn/problems/longest-common-prefix/)
struct Solution;

// 原地切片上判断，只在返回时进行一次堆分配，相比定义一个结果字符串，每次往里push字符性能要高。
impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs.pop().unwrap();
        }
        let first = strs[0].as_bytes();
        let n = first.len();
        for i in 0..n {
            let cur_char = first[i];
            for str in strs.iter().skip(1) {
                let bytes = str.as_bytes();
                if i >= bytes.len() || cur_char != bytes[i] {
                    return strs[0][0..i].to_string();
                }
            }
        }
        strs[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = vec![String::from("a"), String::from("b")];
        let ans = String::new();
        assert_eq!(Solution::longest_common_prefix(strs), ans);
    }
}
