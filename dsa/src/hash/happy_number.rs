/// [202. 快乐数](https://leetcode.cn/problems/happy-number)
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    /// 哈希表记录已经处理过的数
    pub fn is_happy(n: i32) -> bool {
        let mut n = n as usize;
        let mut set = HashSet::new();
        while n != 1 && !set.contains(&n) {
            set.insert(n);
            n = Self::new_num(n);
        }
        n == 1
    }

    /// 各位平方和不超过1000，使用定长数组代替哈希表优化性能
    pub fn is_happy2(n: i32) -> bool {
        let mut n = n as usize;
        let mut visited = [false; 1000];
        while n != 1 {
            if n < 1000 {
                if visited[n] {
                    return false;
                }
                visited[n] = true;
            }
            n = Self::new_num(n);
        }
        true
    }

    /// 每一次取数看成是链表的下一个节点，判断是否是循环链表，用快慢指针。
    pub fn is_happy3(n: i32) -> bool {
        let mut slow = n as usize;
        let mut fast = n as usize;
        while fast != 1 {
            slow = Self::new_num(slow);
            fast = Self::new_num(Self::new_num(fast));
            if fast != 1 && fast == slow {
                return false;
            }
        }
        true
    }

    fn new_num(mut n: usize) -> usize {
        let mut sum = 0;
        while n != 0 {
            let num = n % 10;
            sum += num * num;
            n = n / 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 19;
        let ans = true;
        assert_eq!(Solution::is_happy(n), ans);

        let n = 2;
        let ans = false;
        assert_eq!(Solution::is_happy(n), ans);

        let n = 1;
        let ans = true;
        assert_eq!(Solution::is_happy(n), ans);
    }

    #[test]
    fn test2() {
        let n = 19;
        let ans = true;
        assert_eq!(Solution::is_happy2(n), ans);

        let n = 2;
        let ans = false;
        assert_eq!(Solution::is_happy2(n), ans);

        let n = 1;
        let ans = true;
        assert_eq!(Solution::is_happy2(n), ans);
    }

    #[test]
    fn test3() {
        let n = 19;
        let ans = true;
        assert_eq!(Solution::is_happy3(n), ans);

        let n = 2;
        let ans = false;
        assert_eq!(Solution::is_happy3(n), ans);

        let n = 1;
        let ans = true;
        assert_eq!(Solution::is_happy3(n), ans);
    }
}
