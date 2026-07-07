/// [852. 山脉数组的峰顶索引](https://leetcode.cn/problems/peak-index-in-a-mountain-array/)
pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        if arr.len() < 3 {
            unreachable!();
        }
        let mut lo = 1;
        let mut hi = arr.len() - 2;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let (prev_val, mid_val, next_val) = (arr[mid - 1], arr[mid], arr[mid + 1]);
            if prev_val < mid_val && mid_val > next_val {
                return mid as i32;
            } else if mid_val > next_val {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        unreachable!();
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut lo = 1;
        let mut hi = arr.len() - 2;
        let mut ans = 0;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if arr[mid] > arr[mid + 1] {
                ans = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        ans as i32
    }
}

pub struct Solution3;

impl Solution3 {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut lo = 1;
        let mut hi = arr.len() - 2;
        let mut ans = 0;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if arr[mid] > arr[mid - 1] {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
