/// [150. 逆波兰表达式求值](https://leetcode.cn/problems/evaluate-reverse-polish-notation/)
pub struct Solution;

/// 栈
/// 注意栈容量的计算
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        // 极端情况遇到数字都在左边，符号都在右边的表达式，这时最大长度就是：tokens.len() / 2 + 1
        let mut stack = Vec::with_capacity(tokens.len() / 2 + 1);
        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let val = match token.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => unreachable!(),
                    };
                    stack.push(val);
                }
                _ => {
                    let val: i32 = token.parse().unwrap();
                    stack.push(val);
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
