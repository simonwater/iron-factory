/// [9. 回文数](https://leetcode.cn/problems/palindrome-number/)
pub struct Solution;
// 得到逆序数字再判断
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut num = x as i64;
        let mut rev_num = 0i64;
        while num != 0 {
            rev_num = rev_num * 10 + (num % 10);
            num = num / 10;
        }
        rev_num == x as i64
    }
}

pub struct Solution2;

// 只判断一半，0结尾的需要特殊判断
impl Solution2 {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut num = 0;
        while x > num {
            num = num * 10 + (x % 10);
            x = x / 10;
        }
        x == num || x == num / 10
    }
}

pub struct Solution3;

// 转为字符串再判断
impl Solution3 {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let s_bytes = s.as_bytes();
        let mut i = 0;
        let mut j = s_bytes.len() - 1;
        while i < j {
            if s_bytes[i] != s_bytes[j] {
                return false;
            }
            if j == 0 {
                break;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
