/// [146. LRU 缓存](https://leetcode.cn/problems/lru-cache/description/)
use std::collections::HashMap;

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
}

pub struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

impl LRUCache {
    pub fn new(cap: i32) -> Self {
        let mut nodes = Vec::with_capacity(cap as usize + 1);
        let dumy = Node {
            key: 0,
            value: 0,
            prev: 0,
            next: 0,
        };
        nodes.push(dumy);
        LRUCache {
            capacity: cap as usize,
            map: HashMap::with_capacity(cap as usize),
            nodes,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            self.move_to_head(idx);
            return self.nodes[idx].value;
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].value = value;
            self.move_to_head(idx);
        } else {
            if self.map.len() >= self.capacity {
                // 逐出末尾节点
                let tail_idx = self.nodes[0].prev;
                let tail_node = &mut self.nodes[tail_idx];
                self.map.remove(&tail_node.key);

                tail_node.key = key;
                tail_node.value = value;
                self.map.insert(key, tail_idx);
                self.move_to_head(tail_idx);
            } else {
                self.nodes.push(Node {
                    key,
                    value,
                    prev: 0,
                    next: 0,
                });
                let idx = self.nodes.len() - 1;
                self.map.insert(key, idx);
                self.append_to_head(idx);
            };
        }
    }

    fn append_to_head(&mut self, idx: usize) {
        let old_next_idx = self.nodes[0].next;
        self.nodes[idx].prev = 0;
        self.nodes[idx].next = old_next_idx;
        self.nodes[0].next = idx;
        self.nodes[old_next_idx].prev = idx;
    }

    fn remove_node(&mut self, idx: usize) {
        let prev_idx = self.nodes[idx].prev;
        let next_idx = self.nodes[idx].next;
        self.nodes[prev_idx].next = next_idx;
        self.nodes[next_idx].prev = prev_idx;
    }

    fn move_to_head(&mut self, idx: usize) {
        self.remove_node(idx);
        self.append_to_head(idx);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(3, 3);
        assert_eq!(lru.get(2), -1);
        lru.put(4, 4);
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(4), 4);
    }
}
