/// [224. 基本计算器](https://leetcode.cn/problems/basic-calculator/)
/// [227. 基本计算器 II](https://leetcode.cn/problems/basic-calculator-ii/)
/// [772. 基本计算器 III](https://leetcode.cn/problems/basic-calculator-iii/)
struct Solution;

// 支持i32整数减+、-、*、/、（、）的表达式计算
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut s_bytes = s.as_bytes().iter().peekable();
        let mut nums: Vec<i32> = Vec::with_capacity(16);
        let mut ops: Vec<u8> = Vec::with_capacity(16);
        nums.push(0); // 兼容负号开头
        while let Some(&&c) = s_bytes.peek() {
            match c {
                b' ' => {
                    s_bytes.next(); // 消耗空格
                }
                b'0'..=b'9' => {
                    // 处理数字
                    let mut num = 0;
                    while let Some(&val) = s_bytes.next_if(|&&next| next >= b'0' && next <= b'9') {
                        num = num * 10 + (val - b'0') as i32;
                    }
                    nums.push(num);
                }
                b'(' => {
                    ops.push(c);
                    s_bytes.next();
                    while let Some(_) = s_bytes.next_if_eq(&&b' ') {}
                    if s_bytes.peek() == Some(&&b'-') {
                        nums.push(0);
                    }
                }
                b')' => {
                    while ops.last() != Some(&b'(') {
                        Self::calc(&mut ops, &mut nums);
                    }
                    ops.pop(); // 弹出'(' 
                    s_bytes.next();
                }
                b'+' | b'-' | b'*' | b'/' => {
                    // 执行计算
                    while let Some(&prev) = ops.last() {
                        if prev != b'(' && Self::priority(prev) >= Self::priority(c) {
                            Self::calc(&mut ops, &mut nums);
                        } else {
                            break;
                        }
                    }
                    ops.push(c);
                    s_bytes.next();
                }
                _ => unreachable!(),
            }
        }
        while !ops.is_empty() {
            Self::calc(&mut ops, &mut nums);
        }

        nums.pop().unwrap()
    }

    fn calc(ops: &mut Vec<u8>, nums: &mut Vec<i32>) {
        let b = nums.pop().unwrap();
        let a = nums.pop().unwrap();
        let op = ops.pop().unwrap();
        match op {
            b'+' => nums.push(a + b),
            b'-' => nums.push(a - b),
            b'*' => nums.push(a * b),
            b'/' => nums.push(a / b),
            _ => unreachable!(),
        }
    }

    fn priority(op: u8) -> i32 {
        match op {
            b'+' | b'-' => 10,
            b'*' | b'/' => 20,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("(1+(4+5+2)-3)+(6+8)");
        assert_eq!(Solution::calculate(s), 23);

        let s = String::from("1 + 1");
        assert_eq!(Solution::calculate(s), 2);

        let s = String::from(" (2-1) + 2 ");
        assert_eq!(Solution::calculate(s), 3);

        let s = String::from("1-(     -2)");
        assert_eq!(Solution::calculate(s), 3);

        let s = String::from("1+((2))");
        assert_eq!(Solution::calculate(s), 3);
    }
}
