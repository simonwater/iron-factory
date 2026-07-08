# Rust Data Structure & Algorithm 知识总结

> 基于 LeetCode Rust 题解的算法、数据结构、模式与 Rust 语言技巧全面回顾手册
>
> 点击题号可直接跳转到对应源码文件
> 
> 2026-07-03

---

## 目录

- [一、动态规划 (DP)](#一动态规划-dp)
- [二、图论 (Graph)](#二图论-graph)
- [三、二叉树 (Binary Tree)](#三二叉树-binary-tree)
- [四、链表 (Linked List)](#四链表-linked-list)
- [五、回溯 (Backtrack)](#五回溯-backtrack)
- [六、数组 (Array)](#六数组-array)
- [七、二分搜索 (Binary Search)](#七二分搜索-binary-search)
- [八、栈 (Stack)](#八栈-stack)
- [九、字符串 (String)](#九字符串-string)
- [十、堆 (Heap)](#十堆-heap)
- [十一、哈希表 (Hash)](#十一哈希表-hash)
- [十二、Trie (前缀树)](#十二trie-前缀树)
- [十三、数学与位运算 (Math & Binary)](#十三数学与位运算-math--binary)
- [十四、跨领域 Rust 模式汇总](#十四跨领域-rust-模式汇总)
- [十五、算法模式索引](#十五算法模式索引)

---

## 一、动态规划 (DP)

### 1.1 线性 DP / 斐波那契型

| 题目 | 状态定义 | 转移方程 | 空间 | Rust 技巧 |
|------|---------|---------|------|-----------|
| [70. 爬楼梯](src/dp/climb_stairs.rs) | `a=f(n-2)`, `b=f(n-1)` | `(a,b) = (b, a+b)` | O(1) | **元组交换** `(a, b) = (b, a + b)` |
| [91. 解码方法](src/dp/decode_ways.rs) | `a=dp[i-2]`, `b=dp[i-1]` | 条件分支: 单字符 + 双字符 | O(1) | `s.as_bytes()` + 字节算术 `bytes[i] - b'0'`；提前返回 0 |
| [198. 打家劫舍 I](src/dp/house_robber_i_ii.rs) | `a=不抢前一个`, `b=抢前一个` | `b = max(num+a, b)` | O(1) | 元组交换；`iter().skip(1)` |
| [198. 打家劫舍 II(环形)](src/dp/house_robber_i_ii.rs) | 同上 | `max(rob(0..n-2), rob(1..n-1))` | O(1) | 切片 `&nums[1..]`、`&nums[..n-1]` 避免额外数组 |
| [337. 打家劫舍 III(树)](src/dp/house_robber_iii.rs) | 后序返回 `(rob, not_rob)` | `rob = val+not_rob_l+not_rob_r` | O(h) | `Rc<RefCell<>>`；`as_ptr() as usize` 做 HashMap 键 |

### 1.2 Kadane 算法及其变体

| 题目 | 核心思想 | 关键变量 | Rust 技巧 |
|------|---------|---------|-----------|
| [53. 最大子数组和](src/dp/maximum_subarray.rs) | `cur = max(num, cur+num)` | `cur`, `ans` | `num.max(cur + num)` 链式调用 |
| [918. 环形子数组最大和](src/dp/max_subarray_sum_circular.rs) | `max(最大子数组和, 总和-最小子数组和)` | `max_sub`, `min_sub`, `sum` | 单次遍历同时计算 max/min；全负判断 |
| [152. 乘积最大子数组](src/dp/max_product.rs) | 同时追踪 `pre_max` 和 `pre_min` | `pre_max`, `pre_min` | 缓存旧值；`num.max((num*a).max(num*b))` 嵌套 max |

### 1.3 2D DP（经典）

| 题目 | dp 含义 | 转移 | 空间优化 |
|------|---------|------|----------|
| [72. 编辑距离](src/dp/edit_distance.rs) | `dp[i][j]` = s1[0..i]→s2[0..j] 最小操作 | `min(替换, 删除, 插入) + 1` | 2D 数组，无优化 |
| [1143. 最长公共子序列](src/dp/longest_common_subsequence.rs) | `dp[i][j]` = LCS 长度 | 匹配: `dp[i-1][j-1]+1`；不匹配: `max(dp[i-1][j], dp[i][j-1])` | **1D 压缩 + `left_top` 变量**保存对角线值 |
| [97. 交错字符串](src/dp/interleave_string.rs) | `dp[i][j]` = s3[0..i+j] 是否由 s1[0..i] + s2[0..j] 交错 | `dp[i][j] = (s1匹配 && dp[i-1][j]) \|\| (s2匹配 && dp[i][j-1])` | 2D + DFS 记忆化 (`Vec<Vec<Option<bool>>>`) |
| [221. 最大正方形](src/dp/maximal_square.rs) | `dp[r][c]` = 以(r,c)为右下角的最大正方形边长 | `min(上, 左, 左上) + 1` | **1D + `prev` 变量**保存左上角值 |
| [64. 最小路径和](src/dp/min_path_sum.rs) | `dp[c]` = 到当前行列c的最小路径和 | `min(dp[c], dp[c-1]) + val` | 原地 1D 覆盖（无额外变量） |
| [120. 三角形最小路径和](src/dp/triangle.rs) | `dp[j]` = 从底到(i,j)的最小路径和 | `dp[j] = min(dp[j], dp[j+1]) + val` | **从底向上**；(0..n-1).rev() |

### 1.4 状态机 DP（股票系列）

| 题目 | 交易次数 | 状态数 | 关键技巧 |
|------|---------|--------|----------|
| [121. 买卖股票 I](src/dp/buy_and_sell_stock.rs) | 1 | `prev`(最低价), `ans`(最大利润) | O(1) 单次遍历 |
| [122. 买卖股票 II](src/dp/buy_and_sell_stock_ii.rs) | 无限 | `cash`, `stock` 两状态 | 元组同时更新 `(cash, stock) = (new_cash, new_stock)` |
| [123. 买卖股票 III](src/dp/buy_and_sell_stock_iii.rs) | ≤2 | `buy1, sell1, buy2, sell2` 四状态 | 四元组元组赋值 |
| [188. 买卖股票 IV](src/dp/buy_and_sell_stock_iv.rs) | ≤k | `buys[k+1]`, `sells[k+1]` | **泛化到向量**：`vec![-prices[0]; k+1]` 初始化，嵌套循环 |

### 1.5 背包问题套系

**核心区分：0/1 背包 vs 完全背包 vs 排列**

| 题目 | 类型 | 求什么 | 循环顺序 | 关键注意 |
|------|------|--------|---------|----------|
| [416. 分割等和子集](src/dp/bag_combine/combine_can_partition.rs) | 0/1 | 是否存在 | — | **1D 逆序**：`(num..=target).rev()` 防止重复使用 |
| [322. 零钱兑换](src/dp/bag_combine/combine_coin_change.rs) | 完全 | 最少硬币数 | 外层金额/内层硬币 | `i32::MAX / 2` 避免溢出；`coins.sort_unstable()` 提前 break |
| [518. 零钱兑换 II](src/dp/bag_combine/combine_coin_change_ii.rs) | 完全 | **组合**数 | **外层硬币/内层金额** | 若外层金额则变成排列！见下方说明 |
| [279. 完全平方数](src/dp/bag_combine/combine_num_squares.rs) | 完全 | 最少数量 | 同 322 | 预计算平方数 vs 内联计算 |
| [377. 组合总和 IV](src/dp/bag_combine/permute_sum_iv.rs) | 完全 | **排列**数 | **外层金额/内层硬币** | 本质是爬楼梯扩展（顺序重要） |

> **🔑 背包最关键的认知**（来自 [518. 零钱兑换 II](src/dp/bag_combine/combine_coin_change_ii.rs) 注释）：
> - 外层硬币、内层金额 → **组合**（每个硬币只考虑一次）
> - 外层金额、内层硬币 → **排列**（每个金额可以从任意硬币结尾）
> - 以 [1,2] 构成 3 为例：组合是 (1,1,1) 和 (1,2) = 2 种；排列是 (1,1,1), (2,1), (1,2) = 3 种

### 1.6 其他 DP

| 题目 | 模式 | 关键点 |
|------|------|--------|
| [5. 最长回文子串](src/dp/longest_palindrome.rs) | 中心扩展 / 2D DP | `dp[i][j] = s[i]==s[j] && dp[i+1][j-1]`；`String::from_utf8` 从字节重构 |
| [32. 最长有效括号](src/dp/longest_valid_parentheses.rs) | 线性 DP | `dp[cnt]` 用 `cnt=i+1` 做 1-indexed；字节常量 `b'('` / `b')'` |
| [55. 跳跃游戏 / 45. 跳跃游戏 II](src/dp/jump_game.rs) | 贪心/BFS | `furthest` 可达性 / `cur_end`+`next` 区间扩展 |
| [300. 最长递增子序列](src/dp/length_of_lis.rs) | DP / 耐心排序 | O(n log n) 用 `tails` 数组 + 手动二分 |

---

## 二、图论 (Graph)

### 2.1 图的四种表示法

| 表示法 | 适用场景 | Rust 实现 | 示例文件 |
|--------|---------|-----------|----------|
| **邻接表** | 通用 | `Vec<Vec<usize>>` 或 `Vec<Vec<(usize, T)>>` | 大多数文件 |
| **邻接矩阵** | 稠密图/给定矩阵 | 直接使用 `Vec<Vec<i32>>` | [547. 省份数量](src/graph/number_of_provinces.rs) |
| **链式前向星** | 高性能/竞赛 | `Vec<Edge>` + `Vec<usize> heads` + `usize::MAX` 哨兵 | [210. 课程表 II](src/graph/course_schedule_ii.rs)、[310. 最小高度树](src/graph/min_height_trees.rs)、[743. 网络延迟](src/graph/network_delay_time.rs) |
| **HashMap 图** | 字符串节点 | `HashMap<&String, usize>` 映射 + `Vec<Vec<(usize, f64)>>` | [399. 除法求值](src/graph/evaluate_division.rs)、[815. 公交路线](src/graph/bus_routes.rs) |

**链式前向星结构：**
```rust
const EMPTY_EDGE: usize = usize::MAX;
struct Edge { to: usize, next: usize /* 可扩展 weight 等字段 */ }
let mut heads = vec![EMPTY_EDGE; n];
// 加边: edge.next = heads[u]; heads[u] = edge_idx;
// 遍历: let mut e = heads[u]; while e != EMPTY_EDGE { ... e = edges[e].next; }
```

### 2.2 算法速查表

| 算法 | 题目 | 关键数据结构 | Rust 特定模式 |
|------|------|-------------|--------------|
| **BFS** | [200.岛屿数量](src/graph/num_islands.rs)、[733.图像渲染](src/graph/flood_fill.rs)、[542.01矩阵](src/graph/update_matrix.rs)、[994.腐烂橘子](src/graph/rotting_oranges.rs)、[752.开锁](src/graph/open_lock.rs)、[127.单词接龙](src/graph/word_ladder.rs)、[433.基因变化](src/graph/min_mutation.rs)、[909.蛇梯棋](src/graph/snakes_and_ladders.rs)、[1091.最短路径](src/graph/shortest_path_binary_matrix.rs) | `VecDeque<(state, step)>` | `HashSet::take()` 零拷贝获取所有权；`[false; 10000]` 定长访问数组；先标记再入队 |
| **多源 BFS** | [542](src/graph/update_matrix.rs)、[994](src/graph/rotting_oranges.rs)、[417](src/graph/pacific_atlantic.rs) | 所有源同时入队 | 原地修改 grid 标记访问；`ans[r][c] > step+1` 松弛 |
| **DFS/淹没** | [200](src/graph/num_islands.rs)、[695.岛屿面积](src/graph/max_area_of_island.rs)、[130.被围绕区域](src/graph/surrounded_regions.rs)、[797.所有路径](src/graph/all_paths_source_target.rs) | 递归 / 回溯 | 方向数组 `[(0,1),(0,-1),(1,0),(-1,0)]`；i32 坐标避免 usize 下溢 |
| **记忆化 DFS** | [329.最长递增路径](src/graph/longest_increasing_path.rs) | `dp[r][c]` 记忆化矩阵 | `ans.max(Self::dfs(...))` 聚合 |
| **Dijkstra** | [743.网络延迟](src/graph/network_delay_time.rs)、[1976.方案数](src/graph/count_paths.rs) | `BinaryHeap<Reverse((dist, node))>` | `Reverse` 转最小堆；`i64::MAX / 2` 防溢出；`state > dist[node]` 跳过陈旧条目 |
| **Bellman-Ford** | [787.最便宜航班](src/graph/find_cheapest_price.rs) | 双重距离数组 `prices`/`tmp` | `clone_from_slice` 快照；严格步数限制 |
| **拓扑排序(BFS)** | [207.课程表](src/graph/course_schedule.rs)、[210.课程表 II](src/graph/course_schedule_ii.rs)、[LCR 114.火星词典](src/graph/alien_dictionary.rs) | 入度数组 + `VecDeque` | `VecDeque::with_capacity(n)`；处理不连通图 |
| **拓扑排序(DFS)** | [207](src/graph/course_schedule.rs)、[210](src/graph/course_schedule_ii.rs) | `states: Vec<u8>` (0/1/2) | `match states[node]`；自定义 `enum State` |
| **拓扑剥离** | [310.最小高度树](src/graph/min_height_trees.rs) | 链式前向星 + 叶子队列 | 逐层剥离直到剩余 ≤2 个节点 |
| **并查集** | [547.省份数量](src/graph/number_of_provinces.rs)、[684.冗余连接](src/graph/redundant_connection.rs) | `parents: Vec<usize>` | 递归路径压缩；`(0..=n).collect()` |
| **Trie + DFS** | [212.单词搜索 II](src/graph/word_search_ii.rs) | `Box<Trie>` + `[Option<Box<Trie>>; 26]` | `as_deref_mut()` 导航；`take()` 移除找到的单词；回溯删叶子 |
| **回溯优化** | [79.单词搜索](src/graph/word_search.rs) | visited 矩阵 + 剪枝 | 字符计数预检；起点/终点频率平衡；`word_bytes.reverse()` |

### 2.3 图论中的 Rust 特有技巧

```rust
// 1. BinaryHeap 模拟最小堆
use std::{cmp::Reverse, collections::BinaryHeap};
let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
heap.push(Reverse((0, start)));
while let Some(Reverse((dist, node))) = heap.pop() { ... }

// 2. HashSet::take() — 获取所有权同时删除（零拷贝）
if let Some(word) = word_set.take(&new_word) {
    q.push_back(word);  // 不需要 clone
}

// 3. String ⇄ bytes 零拷贝转换
let bytes = s.into_bytes();           // String → Vec<u8> (所有权转移)
let s = String::from_utf8(bytes)?;    // Vec<u8> → String
let s = std::str::from_utf8(&bytes)?; // &[u8] → &str (零拷贝)

// 4. 固定大小数组做 visited 集合
let mut visited = [false; 10000];  // 栈分配，O(1) 访问，无哈希开销

// 5. 方向数组
const D4: [(i32, i32); 4] = [(0,1), (0,-1), (1,0), (-1,0)];
const D8: [(i32, i32); 8] = [(0,1), (0,-1), (1,0), (-1,0), (1,1), (1,-1), (-1,1), (-1,-1)];

// 6. 节点染色枚举
#[derive(PartialEq, Eq, Clone, Copy)]
enum State { UnVisited, Visiting, Visited }
```

---

## 三、二叉树 (Binary Tree)

### 3.1 TreeNode 数据模式

```rust
// 项目使用的标准二叉树节点定义（tree_node.rs）
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
// Rc — 共享所有权（多个指针指向同一节点）
// RefCell — 内部可变性（运行时借用检查代替编译期）
// Option — 可为空（叶子节点的子节点为 None）
```

### 3.2 核心操作速查表

| 操作 | 代码模式 | 说明 |
|------|---------|------|
| 创建节点 | `Rc::new(RefCell::new(TreeNode::new(val)))` | 三层包装 |
| 克隆引用 | `Rc::clone(&node_rc)` | 增加引用计数，不复制节点数据 |
| 只读访问 | `node_rc.borrow()` → `Ref<TreeNode>` | 返回 Ref guard，离开作用域自动释放 |
| 可变访问 | `node_rc.borrow_mut()` → `RefMut<TreeNode>` | 只能有一个活跃的 RefMut |
| 取出子节点 | `node.borrow_mut().left.take()` | **最常用模式**：取出所有权，原地留 None |
| 从 borrow 中递归 | `borrow.left.clone()` | clone Rc 增加引用计数后传出 borrow 范围 |
| 引用不消耗 | `root.as_ref()` | `&Option<Rc<...>>` → `Option<&Rc<...>>` |
| clone 链 | `.as_ref().map(Rc::clone)` | 从引用创建新的 Rc 强引用 |
| 指针比较 | `Rc::ptr_eq(a, b)` | 判断是否为同一个 TreeNode 实例 |
| 释放借用 | `drop(borrow)` | 显式释放 Ref/RefMut，使后续操作可以 borrow |
| and_then | `root.and_then(\|n\| { ... })` | 消费 Option，对 Some 执行闭包并返回新 Option |

### 3.3 遍历方法汇总

| 遍历方式 | 递归实现 | 迭代实现 | 源码 |
|----------|---------|---------|------|
| **前序** | `val → dfs(left) → dfs(right)` | stack push root → pop → push right → push left | [144](src/binary_tree/traversal.rs) |
| **中序** | `dfs(left) → val → dfs(right)` | 沿左链压栈 → pop → 处理 → 转到右子 | [94](src/binary_tree/traversal.rs) |
| **后序** | `dfs(left) → dfs(right) → val` | 前序变体(root→right→left) + `ans.reverse()` | [145](src/binary_tree/traversal.rs) |
| **层序** | `VecDeque` + `sz = q.len()` 分层 | — | [102](src/binary_tree/level_order/level_order.rs) |

### 3.4 关键借用检查器模式

#### 模式 A：作用域块 `{ }` 释放借用（最常用）

```rust
// 场景：需要同时取出 left 和 right，然后立即对 node_rc 做其他操作
let (left, right) = {
    let mut node = node_rc.borrow_mut();  // 获取 RefMut
    (node.left.take(), node.right.take()) // 取出，离开作用域 → RefMut 释放
};
prev.borrow_mut().right = Some(node_rc.clone()); // OK
```

**使用位置**：[114. 展开为链表](src/binary_tree/flatten_to_linked_list.rs)(全部 4 个解法)、[897. 递增顺序树](src/binary_tree/binary_search_tree/increasing_bst.rs)、[427. 四叉树](src/binary_tree/construct_quad_tree.rs)、[729. 日历](src/binary_tree/binary_search_tree/my_calendar_i.rs)、[662. 二叉树宽度](src/binary_tree/width_of_binary_tree.rs)、[919. 完全二叉树插入](src/binary_tree/complete_binary_tree_inserter.rs)

#### 模式 B：`drop(borrow)` 显式释放

```rust
let mut node = node_rc.borrow_mut();
node.right = Self::delete_node(node.right.take(), key);
drop(node);              // 显式释放 RefMut
return Some(node_rc);    // 现在可以安全移动 node_rc
```

**使用位置**：[450. 删除 BST 节点](src/binary_tree/binary_search_tree/delete_node_in_bst.rs)（多处）、[814. 二叉树剪枝](src/binary_tree/prune_tree.rs)、[LCR 053. BST 中序后继](src/binary_tree/binary_search_tree/inorder_successor.rs)、[653. BST 两数之和](src/binary_tree/binary_search_tree/two_sum_bst.rs)

#### 模式 C：clone 出 borrow 范围

```rust
let borrow = node.borrow();
let left = borrow.left.clone();   // clone Rc → 增加引用计数
let right = borrow.right.clone();
Self::dfs(left);   // 传递所有权
Self::dfs(right);
```

**使用位置**：[104. 最大深度](src/binary_tree/max_depth.rs)、[100. 相同树](src/binary_tree/is_same_tree.rs)、[112/113/437. 路径总和](src/binary_tree/path_sum.rs)、[129. 根到叶数字](src/binary_tree/sum_numbers.rs) 等几乎所有递归函数

#### 模式 D：loop + let 绑定（替代 while let）

```rust
// ❌ 错误：while let 中 visitor 被 borrow 覆盖整个循环体
// ✅ 正确：loop + let 绑定，每轮释放 borrow 后再赋值
loop {
    let next = visitor.borrow().right.as_ref().map(Rc::clone);
    if let Some(next_right) = next {
        visitor = next_right; // next 生命周期已结束，锁已释放
    } else { break; }
}
```

**使用位置**：[114. 展开为链表](src/binary_tree/flatten_to_linked_list.rs)（有详细的 5 种方案对比注释）

#### 模式 E：`take()` 后 struct 重建

```rust
let left = node.left.take();
let right = node.right.take();
drop(node);
if left.is_none() || right.is_none() {
    return left.or(right);  // 优雅的 Option 回退
}
```

**使用位置**：[450. 删除 BST 节点](src/binary_tree/binary_search_tree/delete_node_in_bst.rs)、[814. 二叉树剪枝](src/binary_tree/prune_tree.rs)

### 3.5 BST 专用模式

| 操作 | 关键点 | 源码 |
|------|--------|------|
| 验证 BST | 上下界 `(min, max)` 传递，用 `i64` 避免 `i32::MIN/MAX` 边界 | [98. 验证 BST](src/binary_tree/binary_search_tree/is_valid_bst.rs) |
| BST 查找 | 利用 BST 性质剪枝 | [700. BST 搜索](src/binary_tree/binary_search_tree/search_in_bst.rs) |
| BST 删除 | 找到后继（右子树最小节点） | [450. 删除 BST 节点](src/binary_tree/binary_search_tree/delete_node_in_bst.rs) |
| BST LCA | `p<root<q` 时 root 就是 LCA，O(h) | [235. BST 最近公共祖先](src/binary_tree/binary_search_tree/lowest_common_ancestor.rs) |
| BST 序列化 | 前序即可，反序列化用 `(min, max)` 边界 | [449. BST 序列化](src/binary_tree/binary_search_tree/serde_bst.rs) |

### 3.6 二叉树题目索引

| 题目 | 模式 | 源码 |
|------|------|------|
| [94/144/145. 三种遍历](src/binary_tree/traversal.rs) | 递归 + 迭代双解法 | traversal.rs |
| [100. 相同的树](src/binary_tree/is_same_tree.rs) | tuple match 四臂模式 | is_same_tree.rs |
| [101. 对称二叉树](src/binary_tree/is_symmetric.rs) | 交叉递归 `&Option` 引用 | is_symmetric.rs |
| [104. 二叉树最大深度](src/binary_tree/max_depth.rs) | 后序递归 | max_depth.rs |
| [105/106. 从前中/后中序构建](src/binary_tree/build_tree.rs) | HashMap 索引 + 分治 | build_tree.rs |
| [108. 有序数组转 BST](src/binary_tree/binary_search_tree/sorted_array_to_bst.rs) | 二分取中点 + 切片 | sorted_array_to_bst.rs |
| [110. 平衡二叉树](src/binary_tree/is_balanced.rs) | 后序 + -1 哨兵 | is_balanced.rs |
| [112/113/437. 路径总和 I/II/III](src/binary_tree/path_sum.rs) | 回溯 / 前缀和 | path_sum.rs |
| [114. 展开为链表](src/binary_tree/flatten_to_linked_list.rs) | **4 解法 + 借用分析注释** | flatten_to_linked_list.rs |
| [116. 填充右侧指针](src/binary_tree/populating_next_right_pointers.rs) | stub（未完成） | populating_next_right_pointers.rs |
| [124. 最大路径和](src/binary_tree/max_path_sum.rs) | 后序 + `&mut ans` | max_path_sum.rs |
| [129. 根到叶数字求和](src/binary_tree/sum_numbers.rs) | 前序累加 | sum_numbers.rs |
| [199. 右视图](src/binary_tree/right_side_view.rs) | 逆前序(root→right→left) | right_side_view.rs |
| [222. 完全二叉树节点数](src/binary_tree/count_nodes.rs) | 二分高度优化 O(log²n) | count_nodes.rs |
| [226. 翻转二叉树](src/binary_tree/invert_tree.rs) | take() 递归交换 | invert_tree.rs |
| [230. BST 第 K 小](src/binary_tree/binary_search_tree/kth_smallest.rs) | 中序 + 计数 | kth_smallest.rs |
| [235. BST 最近公共祖先](src/binary_tree/binary_search_tree/lowest_common_ancestor.rs) | BST 性质 O(h) | lowest_common_ancestor.rs(BST) |
| [236. 二叉树最近公共祖先](src/binary_tree/lowest_common_ancestor.rs) | 后序 + `Rc::ptr_eq` | lowest_common_ancestor.rs |
| [297. 序列化反序列化](src/binary_tree/serialize_and_deserialize.rs) | 前序 + `*` null 标记 | serialize_and_deserialize.rs |
| [427. 构建四叉树](src/binary_tree/construct_quad_tree.rs) | 分治 + 四向 Rc | construct_quad_tree.rs |
| [449. BST 序列化](src/binary_tree/binary_search_tree/serde_bst.rs) | 前序无 null + 边界恢复 | serde_bst.rs |
| [450. 删除 BST 节点](src/binary_tree/binary_search_tree/delete_node_in_bst.rs) | **drop(node) 模式精华** | delete_node_in_bst.rs |
| [513. 找树左下角值](src/binary_tree/level_order/find_bottom_left_value.rs) | BFS 右先入队 / DFS 深度 | find_bottom_left_value.rs |
| [515. 每行最大值](src/binary_tree/level_order/largest_values.rs) | BFS / DFS 双解法 | largest_values.rs |
| [530. BST 最小绝对差](src/binary_tree/binary_search_tree/minimum_difference.rs) | 中序 + `i32::MIN/2` | minimum_difference.rs |
| [543. 二叉树直径](src/binary_tree/diameter_of_binary_tree.rs) | 后序 + `&mut ans` | diameter_of_binary_tree.rs |
| [572. 子树判断](src/binary_tree/is_subtree.rs) | 暴力 / 高度剪枝双解法 | is_subtree.rs |
| [617. 合并二叉树](src/binary_tree/merge_trees.rs) | 递归 + `root1.or(root2)` | merge_trees.rs |
| [637. 层平均值](src/binary_tree/level_order/average_of_levels.rs) | BFS + sum/sz | average_of_levels.rs |
| [653. BST 两数之和](src/binary_tree/binary_search_tree/two_sum_bst.rs) | HashSet / 双栈双指针 | two_sum_bst.rs |
| [662. 二叉树最大宽度](src/binary_tree/width_of_binary_tree.rs) | 索引编号 + bias 防溢出 | width_of_binary_tree.rs |
| [700. BST 搜索](src/binary_tree/binary_search_tree/search_in_bst.rs) | 递归 and_then / 迭代 | search_in_bst.rs |
| [729. 我的日程安排 I](src/binary_tree/binary_search_tree/my_calendar_i.rs) | 自定义 BST 节点 | my_calendar_i.rs |
| [814. 二叉树剪枝](src/binary_tree/prune_tree.rs) | `and_then` + `drop(borrow)` | prune_tree.rs |
| [863. 距离 K 的节点](src/binary_tree/distance_k_in_tree.rs) | **值做数组索引** + parent 数组 | distance_k_in_tree.rs |
| [897. 递增顺序搜索树](src/binary_tree/binary_search_tree/increasing_bst.rs) | 3 解法：中序/逆中序/迭代 | increasing_bst.rs |
| [919. 完全二叉树插入器](src/binary_tree/complete_binary_tree_inserter.rs) | `VecDeque` 插入候选 | complete_binary_tree_inserter.rs |
| [LCR 053. BST 中序后继](src/binary_tree/binary_search_tree/inorder_successor.rs) | **5 种解法** | inorder_successor.rs |

---

## 四、链表 (Linked List)

### 4.0 所有权模型

链表是 Rust 中"最难"的数据结构，因为其所有权是非线性的。项目中摸索出了两条路径：

| 路径 | 适用场景 | 代价 |
|------|---------|------|
| `Option<Box<ListNode>>` + `take()`/`as_mut()` | 所有权链式操作（反转、合并、排序） | 需频繁 `take()` |
| `Vec<Node>` + `usize` 索引模拟指针 | 需要随机访问/双向链表（LRU） | 手动管理索引 |

### 4.1 节点定义与基础模式

```rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
// Box — 堆分配，确定大小（递归类型必需）
// Option — 表示链表结尾
```

### 4.2 核心操作速查表

| 操作 | 代码模式 | 说明 |
|------|---------|------|
| 创建节点 | `Box::new(ListNode { val, next: None })` | |
| 哑节点 | `let mut dummy = ListNode::new(0);` | 简化头节点操作 |
| 不可变遍历 | `head.as_deref()` → `Option<&ListNode>` | 借用遍历，不消耗 |
| 可变遍历 | `head.as_deref_mut()` → `Option<&mut ListNode>` | 可修改节点 |
| 取下节点 | `node.next.take()` | 取出所有权，原地留 None |
| 消费遍历 | `while let Some(mut node) = head { head = node.next.take(); ... }` | 消耗链表 |
| 引用链 | `prev.next.as_deref_mut().unwrap()` | 穿过 Option→Box→&mut |
| 指针相等 | `std::ptr::eq(a, b)` | 判断两个引用指向同一地址 |
| `or` 合并 | `l1.or(l2)` | 选择非 None 的值 |

### 4.3 链表操作模式分类

| 模式 | 题目及源码 | 关键技巧 |
|------|-----------|----------|
| **反转链表** | [206. 反转链表](src/linkedlist/reverse_linked_list.rs)、[92. 反转链表 II](src/linkedlist/reverse_linked_list.rs) | `prev = Some(node)` 模式；分段反转 + 缝合 |
| **成组反转** | [25. K 组翻转](src/linkedlist/reverse_k_group.rs) | 探测→断链→反转→缝合→推进 五步管线 |
| **快慢指针** | [141. 环检测](src/linkedlist/linked_list_cycle.rs)、[876. 链表中点](src/linkedlist/middle_of_the_linked_list.rs)、[234. 回文链表](src/linkedlist/palindrome_linked_list.rs) | `fast` 两步、`slow` 一步；`ptr::eq` 判断环 |
| **哑节点法** | [21. 合并](src/linkedlist/merge_two_sorted_lists.rs)、[86. 分隔](src/linkedlist/partition_list.rs)、[24. 交换](src/linkedlist/swap_pairs.rs)、[147. 插入排序](src/linkedlist/insertion_sort_list.rs) | 双哑节点分别收集，最后连接 |
| **LRU Cache** | [146. LRU 缓存](src/linkedlist/lru_cache.rs) | **Vec<Node> 模拟双向链表**（索引代替指针）+ HashMap 映射 key→索引；逐出时复用节点 |
| **归并排序** | [148. 排序链表](src/linkedlist/sort_list.rs)、[23. 合并K](src/linkedlist/merge_k_lists.rs) | 快慢找中点 → `take()` 分半 → 递归排序 → 合并 |
| **重排/旋转** | [143. 重排链表](src/linkedlist/reorder_list.rs)、[61. 旋转链表](src/linkedlist/rotate_list.rs) | 找中点→反转后半→交错合并 / 计算 k→断链→重接 |
| **大数加法** | [2. 两数相加](src/linkedlist/add_two_numbers.rs) | 递归 / 迭代 + 进位；栈处理大端序 |

### 4.4 链表中的 Rust 关键模式

**模式 1：take() 断链 — 链表手术的核心操作**
```rust
let next = node.next.take();  // 取出所有权，原位变 None
// 这是所有链表操作的基础：反转、分割、合并、排序、K组翻转都需要
```

**模式 2：可变引用遍历的两种写法**
```rust
// 写法 A：as_mut().unwrap() → 获取 &mut Box<ListNode>
p = p.next.as_mut().unwrap();
// 写法 B：as_deref_mut().unwrap() → 获取 &mut ListNode（少一层间接）
p = p.next.as_deref_mut().unwrap();
```

**模式 3：不可变遍历的两种写法**
```rust
head.as_ref()     // → Option<&Box<ListNode>>
head.as_deref()   // → Option<&ListNode>（更直接）
```

**模式 4：作用域隔离所有权 — 最关键的复合模式**
```rust
let head2 = {
    let mut p = head.as_mut().unwrap();  // 可变借用开始
    // ... 用 p 进行操作 ...
    p.next.take()  // 取出结果，离开作用域 → 借用释放
};
// 现在可以安全地消费 head
```
**这是 [25](src/linkedlist/reverse_k_group.rs)、[143](src/linkedlist/reorder_list.rs)、[234](src/linkedlist/palindrome_linked_list.rs) 的核心技巧。**

**模式 5：消费遍历 vs 借用遍历**
```rust
// 消费遍历（消耗链表所有权）
while let Some(mut node) = head {
    head = node.next.take();
}
// 借用遍历（不消耗）
while let Some(node) = cur.as_deref() {
    cur = &node.next;
}
```

**模式 6：while let 循环中的借用陷阱**
```rust
// ❌ 错误（[61](src/linkedlist/rotate_list.rs) 中被注释掉的代码）
// while let Some(node) = tail.next.as_deref_mut() {
//     tail = node;  // 冲突！
// }
// ✅ 正确：手动展开
while tail.next.is_some() {
    tail = tail.next.as_deref_mut().unwrap(); // 临时借用已释放
}
```

**模式 7：多链合并**
```rust
l1.or(l2)  // 返回非 None 的那个
cur.next = if l1.is_none() { l2 } else { l1 };
```

**模式 8：链表构造 — 头插法（逆序构建）**
```rust
let mut head = None;
for &val in vals.iter().rev() {  // 必须逆序！
    head = Some(Box::new(ListNode { val, next: head }));
}
```

---

## 五、回溯 (Backtrack)

### 5.1 回溯模板体系

| 视角 | 实现方式 | 适用问题 |
|------|---------|---------|
| **"选/不选"视角** | 对每个元素，递归选择或不选择 | 组合总和 I、子集 |
| **"答案位置枚举"视角** | 对答案的每个位置，枚举所有可能候选（使用 `start` 索引去重） | 全排列、组合 |

### 5.2 题目速查表

| 题目 | 枚举视角 | 去重方式 | Rust 技巧 |
|------|---------|---------|-----------|
| [78. 子集](src/backtrack/permute_combination/subsets.rs) | 选/不选 或 位置枚举 | — | `ans.push(path.clone())` 每个前缀都是子集 |
| [90. 子集 II](src/backtrack/permute_combination/subsets_ii.rs) | 两种都有 | **同层去重**：`if i>start && nums[i]==nums[i-1] { continue }` | sort + 同层去重 |
| [77. 组合](src/backtrack/permute_combination/combine_n_k.rs) | 选/不选 或 位置枚举 | 数量剪枝 `n-i+1 < left` | 预分配 `vec![0; k]` |
| [39. 组合总和](src/backtrack/permute_combination/combination_sum.rs) | 选/不选 或 位置枚举 | 无限制重复选择 | `candidates.sort_unstable()` |
| [40. 组合总和 II](src/backtrack/permute_combination/combination_sum_ii.rs) | 选/不选 或 位置枚举 | 同层去重 + 每个元素只能用一次 | sort + 跳过连续重复 |
| [216. 组合总和 III](src/backtrack/permute_combination/combination_sum_iii.rs) | 选/不选 或 位置枚举 | 1..9 固定集合，恰好 k 个数 | 剩余数量剪枝 |
| [46. 全排列](src/backtrack/permute_combination/permutations.rs) | 位置枚举 + used 标记 | visited bool / bitmask / swap | `used_mask & (1<<idx) == 0`；swap 法 O(1) 空间 |
| [47. 全排列 II](src/backtrack/permute_combination/permutations_ii.rs) | swap + 层内去重 | `HashSet` 每层去重 | `used_nums.insert(cur)` 返回 bool |
| [22. 括号生成](src/backtrack/generate_parenthesis.rs) | DFS + 两个计数器 | `left<n` 加 '(' , `right<left` 加 ')' | `vec![0u8; n*2]` 字节缓冲；`String::from_utf8` |
| [17. 电话号码](src/backtrack/letter_combinations.rs) | 位置枚举 | — | `&[&[u8]]` 字节字典 vs `&[&[char]]` |
| [51. N 皇后](src/backtrack/n_queens.rs) | 行位置枚举 + 攻击检测 | `(r1-r2).abs() == (c1-c2).abs()` 对角线 | `vec![b'.'; n]` 字节缓冲渲染 |

### 5.3 回溯去重关键认知

```
同层去重（避免同一位置选相同值）：
  排序 + if i > start && nums[i] == nums[i-1] { continue }

路径去重（避免重复使用同一元素）：
  used 数组 / start 索引递增

组合 vs 排列：
  组合：用 start 索引控制不回头
  排列：用 used 标记但每次从头枚举（或 swap 法）
```

### 5.4 回溯中的 Rust 特有技巧

```rust
// 1. 预分配固定大小 path — [77](src/backtrack/permute_combination/combine_n_k.rs)
let mut path = vec![0; k];

// 2. swap 法全排列 — 零额外空间 [46](src/backtrack/permute_combination/permutations.rs)
nums.swap(start, i);  // 选择
dfs(nums, start + 1, ans);
nums.swap(start, i);  // 回溯

// 3. bitmask 标记 — [46](src/backtrack/permute_combination/permutations.rs)
if used_mask & (1 << idx) == 0 { ... }

// 4. Hash 层内去重 — [47](src/backtrack/permute_combination/permutations_ii.rs)
let mut used_nums = HashSet::with_capacity(nums.len() - start);
if !used_nums.insert(cur) { continue; }

// 5. 字节缓冲渲染 N 皇后 — [51](src/backtrack/n_queens.rs)
let mut row = vec![b'.'; n]; row[col] = b'Q';
String::from_utf8(row).unwrap()
```

---

## 六、数组 (Array)

### 6.1 题目速查表

| 题目 | 算法模式 | 时间 | Rust 技巧 |
|------|---------|------|-----------|
| [1. 两数之和](src/array/two_sum.rs) | HashMap | O(n) | `HashMap::with_capacity(n)`；`entry` API |
| [167. 两数之和 II](src/array/two_sum.rs) | 双指针 | O(n) | 排序数组左右夹逼 |
| [15. 三数之和](src/array/three_sum.rs) | 排序 + 双指针 | O(n²) | `sort_unstable()`；跳过重复优化；提前 break |
| [16. 最接近三数之和](src/array/three_sum_closest.rs) | 排序 + 双指针 | O(n²) | `(sum-target).abs()` 比较距离 |
| [11. 盛水](src/array/container_with_most_water.rs) | 双指针 | O(n) | `heights[l].min(heights[r])` |
| [42. 接雨水](src/array/trap.rs) | 双指针（左右最大高度） | O(n) | `left_max.max(height[l])` |
| [238. 除自身外乘积](src/array/product_except_self.rs) | 前后缀乘积 | O(n) | `vec![1; n]` + `(0..n-1).rev()` |
| [560. 和为 K 的子数组](src/array/subarray_sum_equals_k.rs) | 前缀和 + HashMap | O(n) | `map.entry(sum).or_insert(0)` |
| [525. 连续数组](src/array/contiguous_array.rs) | 前缀和(0→-1, 1→+1) | O(n) | 变体前缀和 |
| [128. 最长连续序列](src/array/longest_consecutive.rs) | HashSet | O(n) | `set.contains(&(num-1))` 只从序列起点开始 |
| [169. 多数元素](src/array/majority_element.rs) | 摩尔投票 / 排序取中 | O(n) | 消去不同元素对 |
| [189. 轮转数组](src/array/rotate_array.rs) | 三次反转 | O(n) | `nums[..k].reverse()` 切片反转 |
| [283/27/26/80. 移动零/移除/去重](src/array/move_zeroes.rs) | 双指针原地 | O(n) | `nums.swap(i, j)` |
| [977. 有序数组平方](src/array/sorted_squares.rs) | 双指针（两端向中间） | O(n) | 绝对值比较；从后往前填充 |
| [217/219/220. 存在重复](src/array/contains_duplicate.rs) | HashSet / BTreeMap | O(n)/O(n log k) | `set.insert(num)` 返回 bool；`BTreeMap::range()` |
| [134. 加油站](src/array/gas_station.rs) | 贪心 | O(n) | 总油量检查 + 最小油量点 |

### 6.2 排序子模块

| 题目/文件 | 内容 | 源码 |
|-----------|------|------|
| 快排 | Lomuto 分区 | [quick_sort.rs](src/array/sort/quick_sort.rs) |
| 插入排序 | 标准实现 | [insertion_sort.rs](src/array/sort/insertion_sort.rs) |
| [215. 第K大元素](src/array/sort/kth_largest.rs) | 三路分区快速选择 + `rand` 随机 pivot | kth_largest.rs |
| [75. 颜色分类](src/array/sort/sort_colors.rs) | 荷兰国旗三指针 | sort_colors.rs |

### 6.3 区间子模块

| 题目 | 关键技巧 | 源码 |
|------|---------|------|
| [56. 合并区间](src/array/interval/merge_intervals.rs) | `sort_unstable_by_key(第一个元素)` | merge_intervals.rs |
| [57. 插入区间](src/array/interval/insert_interval.rs) | 一次遍历，`i32::MAX` 标记已插入 | insert_interval.rs |
| [435. 无重叠区间](src/array/interval/non_overlap_intervals.rs) | 按结束时间排序 → 贪心 | non_overlap_intervals.rs |
| [452. 射气球](src/array/interval/find_min_arrow_shots.rs) | 按结束或开始排序，维护交集 | find_min_arrow_shots.rs |
| [228. 汇总区间](src/array/interval/summary_ranges.rs) | `format!("{}->{}", start, end)` | summary_ranges.rs |

---

## 七、二分搜索 (Binary Search)

### 7.1 二分模板体系

| 模板 | 区间 | 循环 | 收缩 |
|------|------|------|------|
| `[lo, hi]` 闭区间 | `lo=0, hi=n-1` | `while lo <= hi` | `lo=mid+1` / `hi=mid-1` |
| `[lo, hi)` 半开区间 | `lo=0, hi=n` | `while lo < hi` | `lo=mid+1` / `hi=mid` |

### 7.2 题目分类

**标准二分：**

| 题目 | 变体类型 | 源码 |
|------|---------|------|
| [704. 二分查找](src/binary_search/search.rs) | 标准。三种实现：usize/isize/[lo,hi) | search.rs |
| [35. 搜索插入位置](src/binary_search/search_insert.rs) | 不存在时返回 `lo` | search_insert.rs |
| [34. 查找首末位置](src/binary_search/search_range.rs) | **上下界二分**；`&[i32]` 优于 `&Vec<i32>` | search_range.rs |
| [69. x 的平方根](src/binary_search/my_sqrt.rs) | `i64` 防溢出 | my_sqrt.rs |
| [278. 第一个错误版本](src/binary_search/first_bad_version.rs) | 记录 `ans` | first_bad_version.rs |

**旋转数组：**

| 题目 | 关键 | 源码 |
|------|------|------|
| [33. 搜索旋转数组](src/binary_search/rotated_sorted_array.rs) | 判断哪半边有序 | rotated_sorted_array.rs |
| [153. 找旋转数组最小值](src/binary_search/rotated_sorted_array.rs) | 比较 `mid` 和 `last` | rotated_sorted_array.rs |

**二分进阶：**

| 题目 | 模式 | 源码 |
|------|------|------|
| [162. 寻找峰值](src/binary_search/find_peak_element.rs) | 比较邻居；`i64::MIN` 哨兵 | find_peak_element.rs |
| [852. 山脉顶峰](src/binary_search/mountain_array.rs) | 三种比较法；元组解构 | mountain_array.rs |
| [658. K 个最接近元素](src/binary_search/k_closest_elements.rs) | 排除法：比较 `arr[mid]` 和 `arr[mid+k]` | k_closest_elements.rs |
| [300. LIS(耐心排序)](src/binary_search/longest_increasing_subsequence.rs) | 手动 `lower_bound` | longest_increasing_subsequence.rs |
| [354. 俄罗斯套娃](src/binary_search/russian_doll_envelopes.rs) | 排序（宽升高降）+ LIS | russian_doll_envelopes.rs |
| [528. 按权重随机](src/binary_search/random_pick_with_weight.rs) | 前缀和 + `rand::Rng` | random_pick_with_weight.rs |
| [981. 时间键值存储](src/binary_search/time_map.rs) | HashMap + 内部二分 | time_map.rs |

### 7.3 二分中的 usize 陷阱与解决

```rust
// 问题：Rust 的 usize 无符号，mid=0 时 hi=mid-1 会 panic

// 方案 A：guard 模式 ([704](src/binary_search/search.rs))
if mid == 0 { break / return -1; }
hi = mid - 1;

// 方案 B：isize 转型 ([34](src/binary_search/search_range.rs))
let (mut lo, mut hi) = (0isize, nums.len() as isize - 1);

// 方案 C：位运算防加法溢出
let mid = lo + ((hi - lo) >> 1);  // 替代 (lo + hi) / 2
```

---

## 八、栈 (Stack)

| 题目 | 模式 | Rust 技巧 | 源码 |
|------|------|-----------|------|
| [20. 有效括号](src/stack/valid_parentheses.rs) | 栈匹配 | `Vec::with_capacity(s.len()/2)`；`match` 优于 if-else | valid_parentheses.rs |
| [155. 最小栈](src/stack/min_stack.rs) | 栈存 `(value, min)` 元组 | `val.min(prev_min)` | min_stack.rs |
| [150. 逆波兰表达式](src/stack/eval_rpn.rs) | 栈求值 | `match token.as_str()`；容量 `tokens.len()/2+1` | eval_rpn.rs |
| [232. 用栈实现队列](src/stack/my_queue.rs) | 双栈 | `while let Some(val) = s1.pop()` | my_queue.rs |
| [735. 行星碰撞](src/stack/asteroid_collision.rs) | 模拟栈 | `while let Some(&prev) = stack.last()`；`prev+aster==0` 防溢出 | asteroid_collision.rs |
| [739. 每日温度](src/stack/daily_temperatures.rs) | **单调递减栈** | `(i, temp)` 元组入栈 | daily_temperatures.rs |
| [394. 字符串解码](src/stack/decode_string.rs) | 双栈（数字+字符串） | `reserve` + `push_str` 减少分配 | decode_string.rs |
| [844. 退格字符串](src/stack/backspace_compare.rs) | 栈 或 双指针逆序 | `as_bytes()`；`skip_cnt` 追踪 | backspace_compare.rs |
| [224/227/772. 计算器](src/stack/basic_calculator.rs) | **双栈 + 优先级** | `Peekable`；`next_if()`；`^` 右结合；一元负号 `push 0` | basic_calculator.rs |

### 计算器实现要点 ([basic_calculator.rs](src/stack/basic_calculator.rs))

```rust
// 运算符优先级：(左结合优先级, 右结合优先级)
fn priority(op: u8) -> (i32, i32) {
    match op {
        b'+' | b'-' => (10, 15),
        b'*' | b'/' => (20, 25),
        b'^' => (35, 30),    // 高优先级 + 右结合（左>右）
        _ => unreachable!(),
    }
}
// Peekable 前瞻迭代器
let mut s_bytes = s.as_bytes().iter().peekable();
s_bytes.next_if(|&&next| next >= b'0' && next <= b'9')  // 条件消费
```

---

## 九、字符串 (String)

### 9.1 滑动窗口体系

| 题目 | 窗口类型 | 数据结构 | Rust 技巧 | 源码 |
|------|---------|---------|-----------|------|
| [3. 无重复最长子串](src/string/length_of_longest_substring.rs) | 可变窗口 | HashSet / `[i32; 128]` | 定长数组记录上次位置 | length_of_longest_substring.rs |
| [76. 最小覆盖子串](src/string/min_window.rs) | 可变窗口 | `[i32; 60]` | `formed`/`required` 计数避免全扫描 | min_window.rs |
| [438. 找字母异位词](src/string/find_anagrams.rs) | 固定窗口 | `[i32; 26]` 双数组 | **数组相等比较** `s_cnt == p_cnt` | find_anagrams.rs |
| [424. 替换后最长重复](src/string/character_replacement.rs) | 可变窗口 | `[i32; 26]` | 窗口只缩 1 格 | character_replacement.rs |

### 9.2 回文子模块

| 题目 | 方法 | 源码 |
|------|------|------|
| [125. 验证回文串](src/string/palindrome/is_palindrome.rs) | 双指针 / 迭代器一行 | is_palindrome.rs |
| [680. 验证回文串 II](src/string/palindrome/valid_palindrome.rs) | 双指针 + 跳过一个 | valid_palindrome.rs |
| [409. 最长回文串(构造)](src/string/palindrome/longest_palindrome.rs) | 字符计数 | longest_palindrome.rs |
| [5. 最长回文子串](src/string/palindrome/longest_palindrome.rs) | 中心扩展 | longest_palindrome.rs |
| [647. 回文子串总数](src/string/palindrome/palindromic_substrings.rs) | 中心扩展 / DP | palindromic_substrings.rs |

### 9.3 其他字符串题

| 题目 | 模式 | Rust 技巧 | 源码 |
|------|------|-----------|------|
| [49. 字母异位词分组](src/string/group_anagrams.rs) | HashMap 键设计 | **`[u8; 26]` 做 HashMap 键**（定长数组自动 Eq+Hash） | group_anagrams.rs |
| [14. 最长公共前缀](src/string/longest_common_prefix.rs) | 逐字符比较 | 返回切片 `strs[0][0..i]` 而非新建 String | longest_common_prefix.rs |

### 9.4 字符串中的 Rust 性能模式

```rust
// 1. [u8; 26] 定长数组做 HashMap 键（自动 Eq+Hash）
let mut key = [0u8; 26];
for &c in s.as_bytes() { key[(c - b'a') as usize] += 1; }
map.entry(key).or_insert(vec![]).push(s);  // 零堆分配键！

// 2. [i32; 26] 数组可直接 == 比较（PartialEq）— [438](src/string/find_anagrams.rs)
if s_cnt == p_cnt { ... }

// 3. 一行迭代器回文 — [125](src/string/palindrome/is_palindrome.rs)
clean_bytes.iter().filter(|&&b| b.is_ascii_alphanumeric())
    .map(|&b| b.to_ascii_lowercase())
    .clone().eq(clean_bytes.rev())  // .clone() 只克隆迭代器指针，开销为 0

// 4. .zip().all() 锁步数组比较 — [76](src/string/min_window.rs)
s.iter().zip(t.iter()).all(|(&a, &b)| a >= b)

// 5. &str 切片零拷贝 — [290](src/hash/word_pattern.rs)
// HashMap<u8, &str> — u8 键避免 UTF-8 解码，&str 避免字符串拷贝
```

---

## 十、堆 (Heap)

### 10.1 堆操作核心模式

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// BinaryHeap 默认是大顶堆；Reverse 包装 → 小顶堆
let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
min_heap.push(Reverse(val));
let Reverse(top) = min_heap.pop().unwrap();

// peek_mut() — 就地修改堆顶（避免 pop+push 两次重排）
if let Some(mut top) = heap.peek_mut() {
    if *top > new_val { *top = new_val; }
}
```

### 10.2 题目速查表

| 题目 | 堆类型 | 技巧 | 源码 |
|------|--------|------|------|
| [215. 第K大元素](src/heap/kth_largest.rs) | 小顶堆 size=k | 只保留 k 个最大元素 | kth_largest.rs |
| [973. 最接近原点的K个点](src/heap/k_closest.rs) | 大顶堆 (距离,点) | 三种变体：条件push/push+pop/`peek_mut()` | k_closest.rs |
| [658. K个最接近元素](src/heap/k_closest_elements.rs) | 大顶堆 (距离,数) | 堆 + 双指针排除法 | k_closest_elements.rs |
| [295. 数据流中位数](src/heap/median_finder.rs) | **双堆**：大顶堆(左)+小顶堆(右) | 平衡 `left_max.len() == right_min.len()` | median_finder.rs |
| [23. 合并K个升序链表](src/heap/merge_k_lists.rs) | 小顶堆 / **分治** | **自定义 `Ord`** 实现 | merge_k_lists.rs |
| [692. 前K个高频单词](src/heap/top_k_frequent_word.rs) | 小顶堆 + 自定义比较 | 自定义 `Ord` (频率升序,单词降序) | top_k_frequent_word.rs |

### 10.3 自定义 Ord 用于 BinaryHeap

```rust
// [23](src/heap/merge_k_lists.rs)、[692](src/heap/top_k_frequent_word.rs)
#[derive(Eq, PartialEq)]
struct MyNode(Box<ListNode>);
impl Ord for MyNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.val.cmp(&self.0.val)  // 逆序 → 最小堆
    }
}
impl PartialOrd for MyNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

---

## 十一、哈希表 (Hash)

| 题目 | 模式 | Rust 技巧 | 源码 |
|------|------|-----------|------|
| [41. 缺失的第一个正数](src/hash/first_missing_positive.rs) | HashSet / **循环置换** | `nums.swap(i, cur-1)` | first_missing_positive.rs |
| [202. 快乐数](src/hash/happy_number.rs) | HashSet / [bool; 1000] / Floyd 判圈 | `% 10` / `/ 10` 提取各位数 | happy_number.rs |
| [242. 有效字母异位词](src/hash/is_anagram.rs) | `[i32; 26]` 频率数组 | `cnt.iter().all(\|&a\| a == 0)` | is_anagram.rs |
| [205. 同构字符串](src/hash/isomorphic_strings.rs) | 双向 HashMap | `*map.entry(s).or_insert(t) != t` | isomorphic_strings.rs |
| [380. O(1)插入删除随机](src/hash/randomized_set.rs) | **HashMap + Vec + swap 删除** | `swap` + `pop` 实现 O(1) 删除 | randomized_set.rs |
| [383. 赎金信](src/hash/ransom_note.rs) | `[i32; 26]` | 三种变体：`for_each` + `all` 函数式 | ransom_note.rs |
| [290. 单词规律](src/hash/word_pattern.rs) | 双向 HashMap (`u8`→`&str`) | `&str` 切片零拷贝；`split_whitespace()` | word_pattern.rs |

**关键技巧**：凡是字符频率统计，优先用 `[i32; 26]` 或 `[i32; 128]` 定长数组，避免 HashMap 堆分配开销。

---

## 十二、Trie (前缀树)

### 12.1 Trie 数据结构 ([208](src/trie/trie.rs))

```rust
pub struct Trie {
    is_end: bool,
    datas: [Option<Box<Trie>>; 26],  // 定长数组 + Box 分配子节点
}
// 关键操作：
// t.datas[idx].as_deref_mut().unwrap()  → &mut Trie (导航)
// t.datas[idx].as_deref()?              → &Trie (查找)
// is_some_and(|t| t.is_end)             → Rust 1.70+ 方法
```

### 12.2 Trie 的应用

| 题目 | 用法 | 源码 |
|------|------|------|
| [208. 实现 Trie](src/trie/trie.rs) | 基础实现：insert / search / startsWith | trie.rs |
| [139. 单词拆分](src/trie/word_break.rs) | Trie + DP，`dp[i]` = s[0..i] 是否可拆分 | word_break.rs |
| [212. 单词搜索 II](src/graph/word_search_ii.rs) | **Trie + 回溯 + 动态剪枝**：`take()` 移除找到的单词、叶子回溯删除 | word_search_ii.rs |

---

## 十三、数学与位运算 (Math & Binary)

| 题目 | 方法 | Rust 技巧 | 源码 |
|------|------|-----------|------|
| [9. 回文数](src/math/palindrome_number.rs) | 反转一半 / 全反转(i64) / 转字符串 | `i64` 防溢出；三解法 | palindrome_number.rs |
| [231. 2的幂](src/binary/power_of_two.rs) | `n & (n-1) == 0` | 位运算一行 | power_of_two.rs |
| [264. 丑数 II](src/math/ugly_number.rs) | **多路归并**：小顶堆 + HashSet | `BinaryHeap<Reverse<i64>>`；三路 push | ugly_number.rs |
| [287. 寻找重复数](src/math/find_duplicate.rs) | Floyd 判圈（数组即链表） | `loop` + `break` | find_duplicate.rs |
| [621. 任务调度器](src/math/task_scheduler.rs) | 公式法 | `.iter().max()` + `.filter().count()` | task_scheduler.rs |
| [191. 位1的个数](src/binary/number_of_1_bits.rs) | `n & 1` + `n >> 1` | 迭代移位 | number_of_1_bits.rs |
| [338. 比特位计数](src/binary/counting_bits.rs) | DP: 奇数=`ans[i-1]+1`, 偶数=`ans[i/2]` | `Vec::with_capacity(n+1)` | counting_bits.rs |
| [67. 二进制求和](src/binary/add_binary.rs) | 模拟加法 → `Vec<u8>` + 反转 | `String::from_utf8(c).unwrap()` | add_binary.rs |

---

## 十四、跨领域 Rust 模式汇总

### 14.1 所有权与借用模式

| 模式 | 说明 | 典型位置 |
|------|------|----------|
| **`take()` 断链** | 取出所有权，原地留 None | 链表(linkedlist/)、二叉树(binary_tree/) |
| **作用域块 `{}`** | 限制 Ref/RefMut 作用域 | [二叉树](src/binary_tree/flatten_to_linked_list.rs) 遍布 |
| **`drop(borrow)`** | 显式释放借用 | [BST 删除](src/binary_tree/binary_search_tree/delete_node_in_bst.rs) |
| **`clone()` 出 borrow** | Rc clone 增加引用计数 | [二叉树递归](src/binary_tree/max_depth.rs) |
| **`as_ref().map(Rc::clone)`** | 从 `&Option<Rc<>>` 安全提升 | [二叉树遍历](src/binary_tree/traversal.rs) |
| **`while let` vs `loop`** | 避免借用延长 | [展开链表](src/binary_tree/flatten_to_linked_list.rs) |

### 14.2 迭代器与集合模式

| 模式 | 代码 | 说明 |
|------|------|------|
| 枚举迭代 | `for (i, &val) in nums.iter().enumerate()` | 同时获取索引和值 |
| 跳过首个 | `nums.iter().skip(1)` | |
| 逆序范围 | `(0..n).rev()` | |
| 逆序范围（背包） | `(num..=target).rev()` | 0/1 背包空间压缩 |
| Peekable | `iter.peekable()` + `peek()` + `next_if()` | [计算器](src/stack/basic_calculator.rs) |
| 预分配 | `Vec::with_capacity(n)` | 遍布全项目 |
| 消费迭代器 | `nums.into_iter()` | 获取所有权迭代 |
| turbofish | `nums.iter().sum::<i32>()` | 类型标注 |

### 14.3 字符串与字节

| 模式 | 代码 | 说明 |
|------|------|------|
| 字节访问 | `s.as_bytes()` | O(1) 无分配 |
| 字节字面量 | `b'a'`, `b'0'`, `b'('`, `b')'` | 模式匹配用 |
| 字节算术 | `bytes[i] - b'0'` | 字符转数字 |
| 字节重建 | `String::from_utf8(vec).unwrap()` | |
| 零拷贝 &str | `std::str::from_utf8(&bytes).unwrap()` | |
| 所有权转移 | `s.into_bytes()` | String → Vec\<u8\> |

### 14.4 类型转换与边界处理

| 模式 | 代码 | 说明 |
|------|------|------|
| i32 → usize | `i as usize` | 索引需要 usize |
| usize → i32 | `n as i32` | 负数范围检查 |
| i32 → i64 防溢出 | `x as i64` | 平方/大数 |
| 安全无穷大 | `i32::MAX / 2` | 避免 +1 溢出 |
| 哨兵值 | `i32::MIN`, `usize::MAX` | 未访问标记 |
| 分配大小 | `vec![0; n + 1]` | +1 防止索引越界 |

### 14.5 集合操作技巧

| 模式 | 代码 | 说明 |
|------|------|------|
| HashMap 计数 | `*map.entry(k).or_insert(0) += 1` | |
| HashMap 取值 | `map.get(&k).copied().unwrap_or(0)` | |
| HashSet 插入检测 | `if set.insert(val) { ... }` | 返回 bool |
| HashSet 获取所有权 | `set.take(&val)` | 返回 `Option<T>` + 删除 |
| 定长数组做键 | `[u8; 26]` 做 HashMap 键 | 定长数组有 Eq+Hash |
| 定长访问数组 | `[false; 10000]` 做 visited | 栈分配，O(1) |

### 14.6 Rust 类型系统陷阱与解决

**陷阱 1：usize 下溢（减法 panic）**
- Guard 模式：`if mid == 0 { break; }`
- isize 转型：`let (mut lo, mut hi) = (0isize, ...)`
- 位运算：`lo + ((hi - lo) >> 1)`
- 参见 [二分搜索](#七二分搜索-binary-search) 和 [search.rs](src/binary_search/search.rs)

**陷阱 2：while let 中的借用延长**
```rust
// ❌ while let 将 borrow 延长到循环体末尾
// ✅ loop { let ...; if let ... } 每轮释放
```
> 详细对比参见 [flatten_to_linked_list.rs](src/binary_tree/flatten_to_linked_list.rs) 注释

**陷阱 3：整数溢出**
- `i32::MAX + 1` → panic → 用 `i32::MAX / 2`
- `i32::MIN.abs()` → panic → 用 `prev + aster > 0`（[735](src/stack/asteroid_collision.rs)）
- 大数用 `i64` 过渡（[69](src/binary_search/my_sqrt.rs)）

**陷阱 4：可变引用与移动冲突**
- `Rc<RefCell<>>` 遇到复杂双向操作时的解决：[LRU Cache](src/linkedlist/lru_cache.rs) 用 Vec+索引

---

## 十五、算法模式索引

| 模式 | 题目及源码 |
|------|-----------|
| **双指针** | [167](src/array/two_sum.rs)、[15](src/array/three_sum.rs)、[16](src/array/three_sum_closest.rs)、[11](src/array/container_with_most_water.rs)、[42](src/array/trap.rs)、[26/27/80/283](src/array/move_zeroes.rs)、[977](src/array/sorted_squares.rs)、[125](src/string/palindrome/is_palindrome.rs)、[680](src/string/palindrome/valid_palindrome.rs)、[658](src/heap/k_closest_elements.rs) |
| **滑动窗口** | [3](src/string/length_of_longest_substring.rs)、[76](src/string/min_window.rs)、[438](src/string/find_anagrams.rs)、[424](src/string/character_replacement.rs)、[219](src/array/contains_duplicate.rs) |
| **前缀和** | [560](src/array/subarray_sum_equals_k.rs)、[525](src/array/contiguous_array.rs)、[437](src/binary_tree/path_sum.rs) |
| **快慢指针** | [141](src/linkedlist/linked_list_cycle.rs)、[876](src/linkedlist/middle_of_the_linked_list.rs)、[234](src/linkedlist/palindrome_linked_list.rs)、[202](src/hash/happy_number.rs)、[287](src/math/find_duplicate.rs) |
| **单调栈** | [739](src/stack/daily_temperatures.rs)、[42](src/array/trap.rs) |
| **分治** | [23](src/linkedlist/merge_k_lists.rs)、[148](src/linkedlist/sort_list.rs)、[108](src/binary_tree/binary_search_tree/sorted_array_to_bst.rs)、[427](src/binary_tree/construct_quad_tree.rs) |
| **BFS** | [200](src/graph/num_islands.rs)、[733](src/graph/flood_fill.rs)、[542](src/graph/update_matrix.rs)、[994](src/graph/rotting_oranges.rs)、[752](src/graph/open_lock.rs)、[127](src/graph/word_ladder.rs)、[433](src/graph/min_mutation.rs)、[909](src/graph/snakes_and_ladders.rs)、[1091](src/graph/shortest_path_binary_matrix.rs)、[102](src/binary_tree/level_order/level_order.rs)、[103](src/binary_tree/level_order/zigzag_level_order.rs)、[310](src/graph/min_height_trees.rs)、[815](src/graph/bus_routes.rs) |
| **DFS** | [200](src/graph/num_islands.rs)、[695](src/graph/max_area_of_island.rs)、[130](src/graph/surrounded_regions.rs)、[329](src/graph/longest_increasing_path.rs)、[797](src/graph/all_paths_source_target.rs)、[417](src/graph/pacific_atlantic.rs)、[79](src/graph/word_search.rs)、[212](src/graph/word_search_ii.rs) |
| **回溯** | [78](src/backtrack/permute_combination/subsets.rs)、[90](src/backtrack/permute_combination/subsets_ii.rs)、[77](src/backtrack/permute_combination/combine_n_k.rs)、[39](src/backtrack/permute_combination/combination_sum.rs)、[40](src/backtrack/permute_combination/combination_sum_ii.rs)、[46](src/backtrack/permute_combination/permutations.rs)、[47](src/backtrack/permute_combination/permutations_ii.rs)、[22](src/backtrack/generate_parenthesis.rs)、[17](src/backtrack/letter_combinations.rs)、[51](src/backtrack/n_queens.rs)、[216](src/backtrack/permute_combination/combination_sum_iii.rs) |
| **拓扑排序** | [207](src/graph/course_schedule.rs)、[210](src/graph/course_schedule_ii.rs)、[310](src/graph/min_height_trees.rs)、[LCR 114](src/graph/alien_dictionary.rs) |
| **Dijkstra** | [743](src/graph/network_delay_time.rs)、[1976](src/graph/count_paths.rs) |
| **Bellman-Ford** | [787](src/graph/find_cheapest_price.rs) |
| **并查集** | [547](src/graph/number_of_provinces.rs)、[684](src/graph/redundant_connection.rs) |
| **Trie** | [208](src/trie/trie.rs)、[139](src/trie/word_break.rs)、[212](src/graph/word_search_ii.rs) |
| **0/1 背包** | [416](src/dp/bag_combine/combine_can_partition.rs) |
| **完全背包** | [322](src/dp/bag_combine/combine_coin_change.rs)、[518](src/dp/bag_combine/combine_coin_change_ii.rs)、[279](src/dp/bag_combine/combine_num_squares.rs)、[377](src/dp/bag_combine/permute_sum_iv.rs) |
| **Kadane** | [53](src/dp/maximum_subarray.rs)、[918](src/dp/max_subarray_sum_circular.rs)、[152](src/dp/max_product.rs) |
| **LCS/LIS** | [1143](src/dp/longest_common_subsequence.rs)、[300](src/dp/length_of_lis.rs)、[354](src/binary_search/russian_doll_envelopes.rs) |
| **编辑距离** | [72](src/dp/edit_distance.rs) |
| **状态机 DP** | [121](src/dp/buy_and_sell_stock.rs)、[122](src/dp/buy_and_sell_stock_ii.rs)、[123](src/dp/buy_and_sell_stock_iii.rs)、[188](src/dp/buy_and_sell_stock_iv.rs) |
| **区间 DP** | [5](src/dp/longest_palindrome.rs)、[647](src/string/palindrome/palindromic_substrings.rs) |
| **树 DP** | [337](src/dp/house_robber_iii.rs)、[124](src/binary_tree/max_path_sum.rs) |
| **摩尔投票** | [169](src/array/majority_element.rs) |
| **快速选择** | [215](src/array/sort/kth_largest.rs) |
| **三路分区** | [75](src/array/sort/sort_colors.rs)、[215](src/array/sort/kth_largest.rs) |
| **二分搜索** | [704](src/binary_search/search.rs)、[35](src/binary_search/search_insert.rs)、[34](src/binary_search/search_range.rs)、[69](src/binary_search/my_sqrt.rs)、[33](src/binary_search/rotated_sorted_array.rs)、[153](src/binary_search/rotated_sorted_array.rs)、[162](src/binary_search/find_peak_element.rs)、[852](src/binary_search/mountain_array.rs) |
| **LRU Cache** | [146](src/linkedlist/lru_cache.rs) |
| **双栈计算器** | [224/227/772](src/stack/basic_calculator.rs) |
| **多路归并** | [23](src/heap/merge_k_lists.rs)、[264](src/math/ugly_number.rs) |
| **双堆** | [295](src/heap/median_finder.rs) |
| **拓扑剥离** | [310](src/graph/min_height_trees.rs) |

---
