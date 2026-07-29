//! [12. 整数转罗马数字](https://leetcode.cn/problems/integer-to-roman/)
//!

pub struct Solution;

const M: [&[&'static str]; 4] = [
    &["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    &["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    &["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    &["", "M", "MM", "MMM"],
];
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut ans = String::with_capacity(16);
        ans.push_str(M[3][(num / 1000) as usize]);
        num = num % 1000;
        ans.push_str(M[2][(num / 100) as usize]);
        num = num % 100;
        ans.push_str(M[1][(num / 10) as usize]);
        num = num % 10;
        ans.push_str(M[0][(num) as usize]);
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
