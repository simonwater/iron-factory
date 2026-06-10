/// [208. 实现 Trie (前缀树)](https://leetcode.cn/problems/implement-trie-prefix-tree/)
const INIT: Option<Box<Trie>> = None;

pub struct Trie {
    pub is_end: bool,
    pub datas: [Option<Box<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        Self {
            is_end: false,
            datas: [INIT; 26],
        }
    }

    pub fn insert(&mut self, word: String) {
        let bytes = word.as_bytes();
        let mut t = self;
        for &c in bytes {
            let idx = (c - b'a') as usize;
            if t.datas[idx].is_none() {
                t.datas[idx] = Some(Box::new(Trie::new()));
            }
            t = t.datas[idx].as_deref_mut().unwrap();
        }
        t.is_end = true;
    }

    pub fn search_str(&self, word: &str) -> bool {
        self.find(word).is_some_and(|t| t.is_end)
    }

    pub fn search(&self, word: String) -> bool {
        self.find(&word).is_some_and(|t| t.is_end)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }

    fn find(&self, s: &str) -> Option<&Trie> {
        let bytes = s.as_bytes();
        let mut t = self;
        for &c in bytes {
            let idx = (c - b'a') as usize;
            if t.datas[idx].is_none() {
                return None;
            } else {
                t = t.datas[idx].as_deref().unwrap();
            }
        }

        Some(t)
    }
}
