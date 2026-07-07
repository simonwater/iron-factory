/// # [179. 最大数](https://leetcode.cn/problems/largest-number/)
///

pub struct Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strs: Vec<String> = nums.into_iter().map(|num| num.to_string()).collect();
        strs.sort_unstable_by(|a, b| {
            let ab = a.as_bytes().iter().chain(b.as_bytes());
            let ba = b.as_bytes().iter().chain(a.as_bytes());
            ba.cmp(ab)
        });

        if strs[0] == "0" {
            return String::from("0");
        }
        strs.concat()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
