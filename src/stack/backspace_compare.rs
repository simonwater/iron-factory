/// [844. 比较含退格的字符串](https://leetcode.cn/problems/backspace-string-compare/description/)

// 栈
struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_result = Self::clean_datas(s.as_bytes());
        let t_result = Self::clean_datas(t.as_bytes());
        s_result == t_result
    }

    fn clean_datas(datas: &[u8]) -> Vec<u8> {
        let mut ans = Vec::with_capacity(datas.len());
        for &c in datas {
            if c == b'#' {
                ans.pop();
            } else {
                ans.push(c);
            }
        }
        ans
    }
}

// 双指针逆向扫描
struct Solution2;
impl Solution2 {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut i = s_bytes.len() as i32 - 1;
        let mut j = t_bytes.len() as i32 - 1;
        while i >= 0 || j >= 0 {
            // 处理i指针
            let mut skip_cnt = 0;
            while i >= 0 {
                if s_bytes[i as usize] == b'#' {
                    skip_cnt += 1;
                    i -= 1;
                } else {
                    if skip_cnt > 0 {
                        skip_cnt -= 1;
                        i -= 1;
                    } else {
                        break;
                    }
                }
            }

            // 处理j指针
            skip_cnt = 0;
            while j >= 0 {
                if t_bytes[j as usize] == b'#' {
                    skip_cnt += 1;
                    j -= 1;
                } else {
                    if skip_cnt > 0 {
                        skip_cnt -= 1;
                        j -= 1;
                    } else {
                        break;
                    }
                }
            }

            if i < 0 && j < 0 {
                return true;
            }
            if i < 0 || j < 0 || s_bytes[i as usize] != t_bytes[j as usize] {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
