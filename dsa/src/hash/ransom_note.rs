/// [383. 赎金信](https://leetcode.cn/problems/ransom-note/description/)
pub struct Solution;

impl Solution {
    pub fn can_construct1(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0; 26];
        let r_bytes = ransom_note.as_bytes();
        let m_bytes = magazine.as_bytes();
        for &c in r_bytes {
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
        }

        for &c in m_bytes {
            let idx = (c - b'a') as usize;
            cnt[idx] -= 1;
        }

        cnt.iter().all(|&a| a <= 0)
    }

    pub fn can_construct2(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0; 26];
        let r_bytes = ransom_note.as_bytes();
        let m_bytes = magazine.as_bytes();

        for &c in m_bytes {
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
        }

        for &c in r_bytes {
            let idx = (c - b'a') as usize;
            cnt[idx] -= 1;

            if cnt[idx] < 0 {
                return false;
            }
        }
        true
    }

    pub fn can_construct3(ransom_note: String, magazine: String) -> bool {
        let mut cnt = [0; 26];
        magazine
            .as_bytes()
            .iter()
            .for_each(|&c| cnt[(c - b'a') as usize] += 1);

        ransom_note.as_bytes().iter().all(|&c| {
            let idx = (c - b'a') as usize;
            cnt[idx] -= 1;
            cnt[idx] >= 0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let ans = false;
        assert_eq!(Solution::can_construct1(ransom_note, magazine), ans);

        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        let ans = false;
        assert_eq!(Solution::can_construct1(ransom_note, magazine), ans);

        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        let ans = true;
        assert_eq!(Solution::can_construct1(ransom_note, magazine), ans);
    }

    #[test]
    fn test2() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let ans = false;
        assert_eq!(Solution::can_construct2(ransom_note, magazine), ans);

        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        let ans = false;
        assert_eq!(Solution::can_construct2(ransom_note, magazine), ans);

        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        let ans = true;
        assert_eq!(Solution::can_construct2(ransom_note, magazine), ans);
    }

    #[test]
    fn test3() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let ans = false;
        assert_eq!(Solution::can_construct3(ransom_note, magazine), ans);

        let ransom_note = String::from("aa");
        let magazine = String::from("ab");
        let ans = false;
        assert_eq!(Solution::can_construct3(ransom_note, magazine), ans);

        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        let ans = true;
        assert_eq!(Solution::can_construct3(ransom_note, magazine), ans);
    }
}
