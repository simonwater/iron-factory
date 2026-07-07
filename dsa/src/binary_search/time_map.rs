/// [981. 基于时间的键值存储](https://leetcode.cn/problems/time-based-key-value-store/)
use std::collections::HashMap;

pub struct TimeMap {
    values_map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self {
            values_map: HashMap::with_capacity(128),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        // 动态数组的创建必须用闭包，换成or_insert的话每次调用都会进行创建
        self.values_map
            .entry(key)
            .or_insert_with(|| Vec::with_capacity(10))
            .push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.values_map.get(&key) {
            return Self::search_upper(values, timestamp);
        }
        String::new()
    }

    fn search_upper(values: &[(i32, String)], target_time: i32) -> String {
        if values.is_empty() {
            return String::new();
        }
        let mut lo = 0;
        let mut hi = values.len() - 1;
        let mut ans = values.len();
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let time = values[mid].0;
            if time == target_time {
                return values[mid].1.clone();
            } else if time > target_time {
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
            } else {
                ans = mid;
                lo = mid + 1;
            }
        }

        if ans == values.len() {
            String::new()
        } else {
            values[ans].1.clone()
        }
    }
}
