/// # [332. 重新安排行程](https://leetcode.cn/problems/reconstruct-itinerary/)
/// 从 "JFK" 开始一笔画串完所有机票，题目表明至少存在一种合理的行程，所以 "死胡同" 最多只有一个。
/// 所以按后序递归完再往结果中添加节点的方式，遇到岔路口，先走哪边都能走完：
///   1. 先走环路，必会回到分叉点，此时还有死胡同没走，环路会在递归栈中挂起等走完死胡同再把自己加入结果，不会先于死胡同入结果
///   2. 先走死胡同，走到头没路了把自己加入结果，递归往上收回，接着走环路，走完环路没别的路了，同样环路把自己加入结果。

pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut g: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::with_capacity(256);
        for mut ticket in tickets {
            let v = ticket.pop().unwrap();
            let u = ticket.pop().unwrap();
            g.entry(u)
                .or_insert_with(|| BinaryHeap::with_capacity(8))
                .push(Reverse(v));
        }
        let mut ans = Vec::with_capacity(g.len());
        Self::dfs(String::from("JFK"), &mut g, &mut ans);
        ans.reverse();
        ans
    }

    /// 错误写法如下。nexts生命周期严格关联到整个while循环，导致无法在递归参数上再次可变借用 g
    /// let nexts = g.get_mut(&cur).unwrap();
    /// while let Some(Reverse(next)) = nexts.pop() {
    ///     Self::dfs(next, g, ans);
    /// }
    ///
    fn dfs(
        cur: String,
        g: &mut HashMap<String, BinaryHeap<Reverse<String>>>,
        ans: &mut Vec<String>,
    ) {
        while let Some(nexts) = g.get_mut(&cur) {
            if let Some(Reverse(next)) = nexts.pop() {
                Self::dfs(next, g, ans);
            } else {
                break;
            }
        }
        ans.push(cur);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
