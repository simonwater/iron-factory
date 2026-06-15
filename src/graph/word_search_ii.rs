/// # [212. 单词搜索 II](https://leetcode.cn/problems/word-search-ii/)
///

struct Solution;

struct Trie {
    word: Option<String>,
    datas: [Option<Box<Trie>>; 26],
    child_cnt: i32,
}
const EMPTY_TRIE: Option<Box<Trie>> = None;

impl Trie {
    fn new() -> Self {
        Self {
            word: None,
            datas: [EMPTY_TRIE; 26],
            child_cnt: 0,
        }
    }

    fn insert(&mut self, word: String) {
        let mut t = self;
        for &c in word.as_bytes() {
            let idx = (c - b'a') as usize;
            if t.datas[idx].is_none() {
                t.datas[idx] = Some(Box::new(Trie::new()));
                t.child_cnt += 1;
            }
            t = t.datas[idx].as_deref_mut().unwrap();
        }
        t.word = Some(word);
    }
}

impl Solution {
    //
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let m = board.len();
        let n = board[0].len();
        let mut root = Trie::new();
        for word in words {
            root.insert(word);
        }
        let mut ans = Vec::with_capacity(32);
        for i in 0..m {
            for j in 0..n {
                Self::dfs(i, j, &mut board, &mut root, &mut ans);
            }
        }

        ans
    }

    fn dfs(r: usize, c: usize, board: &mut Vec<Vec<char>>, trie: &mut Trie, ans: &mut Vec<String>) {
        let cur_char = board[r][c];
        if cur_char == '#' {
            return;
        }
        let idx = (cur_char as u8 - b'a') as usize;
        if let Some(next_trie) = trie.datas[idx].as_deref_mut() {
            if let Some(word) = next_trie.word.take() {
                ans.push(word);
            }

            let m = board.len();
            let n = board[0].len();
            board[r][c] = '#';
            if r > 0 {
                Self::dfs(r - 1, c, board, next_trie, ans);
            }
            if c > 0 {
                Self::dfs(r, c - 1, board, next_trie, ans);
            }
            if r < m - 1 {
                Self::dfs(r + 1, c, board, next_trie, ans);
            }
            if c < n - 1 {
                Self::dfs(r, c + 1, board, next_trie, ans);
            }
            board[r][c] = cur_char;

            if next_trie.child_cnt == 0 {
                // 当前已经是叶子节点，则在父节点中进行删除
                trie.datas[idx] = None;
                trie.child_cnt -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
