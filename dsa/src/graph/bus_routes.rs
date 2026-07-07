/// # [815. 公交路线](https://leetcode.cn/problems/bus-routes/)
///

/// 以车站为节点建图，相邻节点为：所有经过当前车站的bus所经过的车站。
pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    //
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut stop2bus: HashMap<i32, Vec<usize>> = HashMap::with_capacity(32);
        for (bus, stops) in routes.iter().enumerate() {
            for &stop in stops {
                stop2bus
                    .entry(stop)
                    .or_insert_with(|| Vec::with_capacity(32))
                    .push(bus);
            }
        }
        if !stop2bus.contains_key(&source) || !stop2bus.contains_key(&target) {
            return -1;
        }
        let n = routes.len();
        let mut bus_visited = vec![false; n];
        let mut visited_stops: HashSet<i32> = HashSet::with_capacity(1024);
        let mut q = VecDeque::with_capacity(1024);
        q.push_back((source, 0));
        visited_stops.insert(source);
        while let Some((cur_stop, bus_cnt)) = q.pop_front() {
            let buses = stop2bus.get(&cur_stop).unwrap();
            let new_cnt = bus_cnt + 1;
            for &bus in buses {
                if bus_visited[bus] {
                    continue;
                }
                bus_visited[bus] = true;
                for &next_stop in &routes[bus] {
                    if next_stop == target {
                        return new_cnt;
                    }
                    if visited_stops.insert(next_stop) {
                        q.push_back((next_stop, new_cnt));
                    }
                }
            }
        }

        -1
    }
}

/// 以bus为图节点，相邻节点为：当前bus经过的所有车站上经过的bus
pub struct Solution2;

impl Solution2 {
    //
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut stop2bus: HashMap<i32, Vec<usize>> = HashMap::with_capacity(32);
        for (bus, stops) in routes.iter().enumerate() {
            for &stop in stops {
                stop2bus
                    .entry(stop)
                    .or_insert_with(|| Vec::with_capacity(32))
                    .push(bus);
            }
        }
        if !stop2bus.contains_key(&source) || !stop2bus.contains_key(&target) {
            return -1;
        }
        let n = routes.len();
        let mut bus_visited = vec![false; n];
        let mut q = VecDeque::with_capacity(n);
        if let Some(start_buses) = stop2bus.get(&source) {
            for &bus in start_buses {
                bus_visited[bus] = true;
                q.push_back((bus, 1));
            }
        }

        while let Some((cur_bus, bus_cnt)) = q.pop_front() {
            for &stop in &routes[cur_bus] {
                if stop == target {
                    return bus_cnt;
                }
                if let Some(next_buses) = stop2bus.get(&stop) {
                    for &next_bus in next_buses {
                        if !bus_visited[next_bus] {
                            bus_visited[next_bus] = true;
                            q.push_back((next_bus, bus_cnt + 1));
                        }
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
