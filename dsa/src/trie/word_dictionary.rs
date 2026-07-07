/// # [211. 添加与搜索单词](https://leetcode.cn/problems/design-add-and-search-words-data-structure/)
/// 包含一些 '.' ，每个 . 都可以表示任何一个字母
const EMPTY: Option<Box<WordDictionary>> = None;
pub struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_end: bool,
    has_children: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self {
            children: [EMPTY; 26],
            is_end: false,
            has_children: false,
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut t = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            if t.children[i].is_none() {
                t.children[i] = Some(Box::new(WordDictionary::new()));
                t.has_children = true;
            }
            t = t.children[i].as_deref_mut().unwrap();
        }
        t.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::dfs(0, &self, word.as_bytes())
    }

    // 深度优先搜索
    fn dfs(i: usize, p_trie: &WordDictionary, bytes: &[u8]) -> bool {
        if i == bytes.len() {
            // 字符串已经到头，结果取决于字典是否处在一个单词的结尾
            return p_trie.is_end;
        }
        // 字符串还没到头但字典已经没有后代
        if !p_trie.has_children {
            return false;
        }

        if bytes[i] != b'.' {
            let child_idx = (bytes[i] - b'a') as usize;
            if let Some(cur_trie) = p_trie.children[child_idx].as_deref() {
                return Self::dfs(i + 1, cur_trie, bytes);
            } else {
                return false;
            }
        } else {
            // 遍历每个后代
            for child in &p_trie.children {
                if let Some(t) = child.as_deref() {
                    if Self::dfs(i + 1, t, bytes) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
