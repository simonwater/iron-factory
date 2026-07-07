pub struct InsertionSort;
/// 插入排序
impl InsertionSort {
    pub fn sort(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        for i in 1..nums.len() {
            let cur = nums[i];
            let mut j = i as i32 - 1;
            while j >= 0 {
                let j_idx = j as usize;
                if nums[j_idx] > cur {
                    nums[j_idx + 1] = nums[j_idx];
                    j -= 1;
                } else {
                    break;
                }
            }
            nums[(j + 1) as usize] = cur;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![4, 2, 1, 5, 6, 3];
        let ans = vec![1, 2, 3, 4, 5, 6];
        InsertionSort::sort(&mut nums);
        assert_eq!(nums, ans);

        let mut nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let ans = vec![1, 2, 2, 3, 3, 4, 5, 5, 6];
        InsertionSort::sort(&mut nums);
        assert_eq!(nums, ans);

        let mut nums = vec![2, 1];
        let ans = vec![1, 2];
        InsertionSort::sort(&mut nums);
        assert_eq!(nums, ans);

        let mut nums = vec![1];
        let ans = vec![1];
        InsertionSort::sort(&mut nums);
        assert_eq!(nums, ans);

        let mut nums = vec![1, 1, 1];
        let ans = vec![1, 1, 1];
        InsertionSort::sort(&mut nums);
        assert_eq!(nums, ans);
    }
}
