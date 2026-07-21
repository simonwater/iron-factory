//! [648. 单词替换](https://leetcode.cn/problems/replace-words/)
//!

pub struct Solution;

const EMPTY: Option<Box<Trie>> = None;
pub struct Trie {
    datas: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            datas: [EMPTY; 26],
            word: None,
        }
    }

    pub fn add(&mut self, word: String) {
        let mut t = self;
        for &c in word.as_bytes() {
            let i = (c - b'a') as usize;
            // Option接口
            t = t.datas[i].get_or_insert_with(|| Box::new(Trie::new()));
            // 剪枝，树中已经有更短的前缀，则更长的词根可以直接跳过
            if t.word.is_some() {
                return;
            }
        }
        t.word = Some(word);
    }

    pub fn find<'a>(&'a self, s: &'a str) -> &'a str {
        let mut t = self;
        for &c in s.as_bytes() {
            let i = (c - b'a') as usize;
            if let Some(next) = t.datas[i].as_deref() {
                if let Some(word) = next.word.as_deref() {
                    return word;
                }
                t = next;
            } else {
                return s;
            }
        }
        s
    }
}

impl Solution {
    pub fn replace_words(mut dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        dictionary.sort_unstable_by_key(|s| s.len());
        for word in dictionary.into_iter() {
            trie.add(word);
        }

        let mut ans = String::with_capacity(sentence.len());
        let mut left = 0usize;
        for (i, &c) in sentence.as_bytes().iter().enumerate() {
            if c == b' ' {
                let word = trie.find(&sentence[left..i]);
                ans.push_str(word);
                ans.push(' ');
                left = i + 1;
            }
        }
        let last = trie.find(&sentence[left..]);
        ans.push_str(last);
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
