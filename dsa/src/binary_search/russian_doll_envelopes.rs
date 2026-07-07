/// [354. 俄罗斯套娃信封问题](https://leetcode.cn/problems/russian-doll-envelopes/)
pub struct Solution;

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let n = envelopes.len();
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut tails = Vec::with_capacity(n);
        for elp in envelopes {
            let pos = Self::lower_bound(&tails, elp[1]);
            if pos == tails.len() {
                tails.push(elp[1]);
            } else {
                tails[pos] = elp[1];
            }
        }

        tails.len() as i32
    }

    fn lower_bound(nums: &[i32], x: i32) -> usize {
        if nums.is_empty() {
            return 0;
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let mut ans = nums.len();
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] >= x {
                ans = mid;
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
