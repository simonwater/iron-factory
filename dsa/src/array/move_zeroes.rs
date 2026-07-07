pub struct Solution;
/// 双指针原地栈：两个指针一个负责对旧数组做扫描，并选择旧数组中符合条件的数。另一个指针负责建设新数组。
impl Solution {
    /// [283. 移动零](https://leetcode.cn/problems/move-zeroes)
    /// 原地栈 + 双指针
    /// 与常规数组实现栈的方式类似，i表示栈大小，即作为慢指针指向栈顶元素上一个。j是快指针负责遍历数组
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
        }
    }

    /// [27. 移除元素](https://leetcode.cn/problems/remove-element)
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }

    /// [26. 删除有序数组中的重复项](https://leetcode.cn/problems/remove-duplicates-from-sorted-array)
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return n as i32;
        }
        let mut i = 1; // 原地栈大小
        for j in 1..n {
            if nums[j] != nums[i - 1] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }

    /// [80. 删除有序数组中的重复项 II](https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii)
    /// 需要注意判断是否相等时应该同栈顶下方的元素比较，而不能跟快指针前两位的数比较，因为这个位置如果指向原地栈，则可能已经被污染。
    pub fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        let mut i = 2;
        for j in 2..n {
            if nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let ans = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, ans);

        let mut nums = vec![0];
        let ans = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, ans);
    }

    #[test]
    fn test2() {
        let mut nums = vec![3, 2, 2, 3];
        let ans = 2;
        assert_eq!(Solution::remove_element(&mut nums, 3), ans);
        assert_eq!(&nums[..ans as usize], &[2, 2]);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let ans = 5;
        assert_eq!(Solution::remove_element(&mut nums, 2), ans);
        assert_eq!(&nums[..ans as usize], &[0, 1, 3, 0, 4]);
    }

    #[test]
    fn test3() {
        let mut nums = vec![1, 1, 2];
        let ans = 2;
        assert_eq!(Solution::remove_duplicates(&mut nums), ans);
        assert_eq!(&nums[..ans as usize], &[1, 2]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let ans = 5;
        assert_eq!(Solution::remove_duplicates(&mut nums), ans);
        assert_eq!(&nums[..ans as usize], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn test4() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let ans = 5;
        assert_eq!(Solution::remove_duplicates_ii(&mut nums), ans);
        assert_eq!(&nums[..ans as usize], &[1, 1, 2, 2, 3]);

        let mut nums = vec![0, 0, 0, 1, 1, 1, 2, 3, 3];
        let ans = 7;
        assert_eq!(Solution::remove_duplicates_ii(&mut nums), ans);
        assert_eq!(&nums[..ans as usize], &[0, 0, 1, 1, 2, 3, 3]);
    }
}
