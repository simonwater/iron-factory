/// [20. 有效的括号](https://leetcode.cn/problems/valid-parentheses/)
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());
        for &c in s.as_bytes() {
            if c == b'(' {
                stack.push(b')');
            } else if c == b'[' {
                stack.push(b']');
            } else if c == b'{' {
                stack.push(b'}');
            } else {
                if let Some(prev) = stack.pop() {
                    if prev == c {
                        continue;
                    }
                }
                return false;
            }
        }
        stack.is_empty()
    }

    pub fn is_valid2(s: String) -> bool {
        // 提前退出
        if s.len() % 2 != 0 {
            return false;
        }
        let mut stack: Vec<u8> = Vec::with_capacity(s.len() / 2); // 括号是成对的，栈里最多只会同时存在一半的左括号
        for &c in s.as_bytes() {
            match c {
                // 相比if/else，会直接在 CPU 里生成 跳转表（Jump Table） 或者利用位掩码，能直接定位到对应语句
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                _ => {
                    if stack.pop() != Some(c) {
                        // 空的时候结果None也能用来做比较
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("()");
        let ans = true;
        assert_eq!(Solution::is_valid(s), ans);

        let s = String::from("()[]{}");
        let ans = true;
        assert_eq!(Solution::is_valid(s), ans);

        let s = String::from("(]");
        let ans = false;
        assert_eq!(Solution::is_valid(s), ans);

        let s = String::from("([])");
        let ans = true;
        assert_eq!(Solution::is_valid(s), ans);

        let s = String::from("([)]");
        let ans = false;
        assert_eq!(Solution::is_valid(s), ans);
    }

    #[test]
    fn test2() {
        let s = String::from("()");
        let ans = true;
        assert_eq!(Solution::is_valid2(s), ans);

        let s = String::from("()[]{}");
        let ans = true;
        assert_eq!(Solution::is_valid2(s), ans);

        let s = String::from("(]");
        let ans = false;
        assert_eq!(Solution::is_valid2(s), ans);

        let s = String::from("([])");
        let ans = true;
        assert_eq!(Solution::is_valid2(s), ans);

        let s = String::from("([)]");
        let ans = false;
        assert_eq!(Solution::is_valid2(s), ans);
    }
}
