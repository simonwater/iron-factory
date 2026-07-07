/// [228. 汇总区间](https://leetcode.cn/problems/summary-ranges/description/)
pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::new();
        if nums.len() == 0 {
            return ans;
        }
        let mut iter = nums.into_iter();
        let mut start = iter.next().unwrap(); // 只需要两个变量表示区间，能避免堆上内存管理
        let mut end = start;
        // 不依赖索引，可以省去很多边界判断
        for num in iter {
            // 防止溢出
            if end == num - 1 {
                end = num;
            } else {
                ans.push(Self::to_string(start, end));
                start = num;
                end = num;
            }
        }
        ans.push(Self::to_string(start, end));
        ans
    }

    fn to_string(start: i32, end: i32) -> String {
        if start == end {
            start.to_string()
        } else {
            format!("{start}->{end}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let ans = vec!["0->2", "4->5", "7"];
        assert_eq!(Solution::summary_ranges(nums), ans);

        let nums = vec![0, 2, 3, 4, 6, 8, 9];
        let ans = vec!["0", "2->4", "6", "8->9"];
        assert_eq!(Solution::summary_ranges(nums), ans);
    }
}
