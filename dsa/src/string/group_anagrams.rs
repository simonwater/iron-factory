/// [49. 字母异位词分组](https://leetcode.cn/problems/group-anagrams/)
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(16);
        for str in strs {
            let mut bytes = str.as_bytes().to_vec();
            bytes.sort_unstable();
            let key = String::from_utf8(bytes).unwrap();
            map.entry(key)
                .or_insert_with(|| Vec::with_capacity(8))
                .push(str);
        }
        map.into_iter().map(|(_, value)| value).collect()
    }
}

/// 固定长度的原生数组不仅天然实现了 PartialEq（能用 == 比较），还天然实现了 Eq 和 Hash 特征（Trait）
/// 这意味着 [u8; 26] 这种纯粹栈上的固定内存，可以直接无缝充当 HashMap 的 Key
pub struct Solution2;
impl Solution2 {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(16);
        for str in strs {
            let bytes = str.as_bytes();
            let mut key: [u8; 26] = [0; 26];
            for &b in bytes {
                key[(b - b'a') as usize] += 1;
            }

            map.entry(key)
                .or_insert_with(|| Vec::with_capacity(8))
                .push(str);
        }

        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
