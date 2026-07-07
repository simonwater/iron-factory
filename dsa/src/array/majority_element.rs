/// [169. 多数元素](https://leetcode.cn/problems/majority-element/description/)
pub struct Solution;

impl Solution {
    pub fn majority_element_sort(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() / 2]
    }

    pub fn majority_element_vote(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut score = 0;
        for num in nums {
            if score == 0 {
                ans = num;
            }

            if num == ans {
                score += 1;
            } else {
                score -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 3];
        assert_eq!(Solution::majority_element_sort(nums), 3);

        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element_sort(nums), 2);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 3];
        assert_eq!(Solution::majority_element_vote(nums), 3);

        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element_vote(nums), 2);
    }
}
