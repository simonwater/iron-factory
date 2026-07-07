/// [67. 二进制求和](https://leetcode.cn/problems/add-binary/description/)
pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let (mut i, mut j) = (a.len(), b.len());
        let mut c: Vec<u8> = Vec::with_capacity(i.max(j) + 1);
        let mut carry = 0u8;
        while i > 0 || j > 0 || carry > 0 {
            if i > 0 {
                carry += a[i - 1] - b'0';
                i -= 1;
            }
            if j > 0 {
                carry += b[j - 1] - b'0';
                j -= 1;
            }
            c.push((carry % 2 + b'0') as u8);
            carry = carry / 2;
        }
        c.reverse();
        String::from_utf8(c).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = String::from("1010");
        let b = String::from("1011");
        assert_eq!(Solution::add_binary(a, b), "10101");

        let a = String::from("11");
        let b = String::from("1");
        assert_eq!(Solution::add_binary(a, b), "100");
    }
}
