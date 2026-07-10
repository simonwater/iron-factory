//! [926. 将字符串翻转到单调递增](https://leetcode.cn/problems/flip-string-to-monotone-increasing/)
//!
//! # 状态机dp
//! - 无后效性。沿着单向时间轴从左到右平推过去，当前位置做出的决策绝对不会对过去的决策产生任何反向干扰。
//! - 第 i 步的最小代价有且仅有第 i-1 步留下的最终状态能作为输入。第 i-2 步及以前的状态都融合在了 i-1
//! 步的几个状态变量里
//!
//! 同类问题：粉刷房子、股票系列

pub struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let n = s.len();
        if n <= 1 {
            return n as i32;
        }
        let (mut cnt0, mut cnt1) = (0, 0);
        for &c in s.as_bytes() {
            let val = c - b'0';
            if val == 0 {
                // 0
                (cnt0, cnt1) = (cnt0, 1 + cnt0.min(cnt1));
            } else {
                // 1
                (cnt0, cnt1) = (1 + cnt0, cnt0.min(cnt1));
            }
        }
        cnt0.min(cnt1)
    }
}
