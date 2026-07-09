/// # [332. 重新安排行程](https://leetcode.cn/problems/reconstruct-itinerary/)
/// **欧拉通路 Hierholzer算法**
///
/// 从 "JFK" 开始一笔画串完所有机票，题目表明至少存在一种合理的行程，所以 "死胡同" 最多只有一个。
/// 所以规划的路径从起点出发遇到岔路口时，只要保证先走环路，回到路口后最后才进入死胡同，就能完成整个行程。
/// 也就是说规划的路径里，死胡同岔路必须保证在最后。按后序遍历递归处理完所有后继再往路径中 **逆序** 添加节点的方式，遇到岔路口，先遍历哪边都能保证让死胡同在路径最后：
/// 1. 先遍历环路，递归在未来某一刻必会再次到达当前分叉点，此时还有死胡同没遍历，整个环路会在递归栈中挂起等遍历完死胡同再把自己逆序加入路径，不会先于死胡同加入路径
/// 2. 先遍历死胡同，到末尾没路了把自己逆序加入路径中，递归往上回溯到当前岔路节点，接着遍历环路，搜索完环路没别的路了，同样环路在回溯时也把自己逆序加入结果中。同样不会先于死胡同加入路径

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
