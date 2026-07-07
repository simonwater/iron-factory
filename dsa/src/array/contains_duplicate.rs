use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::ops::Bound::Included;

/// [217. 存在重复元素](https://leetcode.cn/problems/contains-duplicate/description/)
/// [219. 存在重复元素 II](https://leetcode.cn/problems/contains-duplicate-ii/description/)
pub struct Solution;

impl Solution {
    // set
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        false
    }

    // set + 定长滑动窗口
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 || nums.len() <= 1 {
            return false;
        }
        let k = std::cmp::min(k as usize, nums.len() - 1);
        let mut set = std::collections::HashSet::with_capacity(k + 1);
        for (i, &num) in nums.iter().enumerate() {
            if !set.insert(num) {
                return true;
            }
            if i >= k {
                set.remove(&nums[i - k]);
            }
        }
        false
    }

    // 有序map + 滑动窗口
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        if index_diff == 0 || nums.len() <= 1 {
            return false;
        }
        let index_diff = std::cmp::min(index_diff as usize, nums.len() - 1);
        let value_diff = value_diff as i64;
        let mut map = BTreeMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let cur_num = num as i64;
            let range = (
                Included(cur_num - value_diff),
                Included(cur_num + value_diff),
            );
            if map.range(range).next().is_some() {
                return true;
            }
            *map.entry(cur_num).or_insert(0) += 1;
            if i >= index_diff {
                let out_num = nums[i - index_diff] as i64;
                // if let Some(count) = map.get_mut(&out_num) {
                //     *count -= 1;
                //     if *count == 0 {
                //         map.remove(&out_num);
                //     }
                // }
                if let Entry::Occupied(mut entry) = map.entry(out_num) {
                    *entry.get_mut() -= 1;
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::contains_duplicate(nums), true);

        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::contains_duplicate(nums), false);

        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(Solution::contains_duplicate(nums), true);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::contains_nearby_duplicate(nums, 3), true);

        let nums = vec![1, 0, 1, 1];
        assert_eq!(Solution::contains_nearby_duplicate(nums, 1), true);

        let nums = vec![1, 2, 3, 1, 2, 3];
        assert_eq!(Solution::contains_nearby_duplicate(nums, 2), false);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3, 1];
        let index_diff = 3;
        let value_diff = 0;
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff),
            true
        );

        let nums = vec![1, 5, 9, 1, 5, 9];
        let index_diff = 2;
        let value_diff = 3;
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff),
            false
        );
    }
}
