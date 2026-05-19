use rand::Rng;
use std::collections::HashMap;

/// [380. O(1) 时间插入、删除和获取随机元素](https://leetcode.cn/problems/insert-delete-getrandom-o1)
struct RandomizedSet {
    val_vec: Vec<i32>,
    idx_map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            val_vec: Vec::new(),
            idx_map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.idx_map.contains_key(&val) {
            return false;
        }
        self.idx_map.insert(val, self.val_vec.len());
        self.val_vec.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idx) = self.idx_map.remove(&val) {
            if idx < self.val_vec.len() - 1 {
                let last_idx = self.val_vec.len() - 1;
                let last_val = self.val_vec[last_idx];
                self.val_vec.swap(idx, last_idx);
                self.idx_map.insert(last_val, idx);
            }
            self.val_vec.pop();
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        // 无需放到结构体属性上，否则get_random方法的mut会影响并发性能
        let mut rng = rand::thread_rng();
        let idx: usize = rng.gen_range(0..self.val_vec.len());
        self.val_vec[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut set = RandomizedSet::new();
        assert_eq!(set.insert(1), true);
        assert_eq!(set.remove(2), false);
        assert_eq!(set.insert(2), true);
        set.get_random(); // getRandom 应随机返回 1 或 2 。
        assert_eq!(set.remove(1), true);
        assert_eq!(set.insert(2), false);
        set.get_random();

        assert_eq!(set.insert(0), true);
        assert_eq!(set.remove(0), true);
        assert_eq!(set.insert(0), true);
    }
}
