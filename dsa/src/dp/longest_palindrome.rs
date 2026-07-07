/// [5. 最长回文子串](https://leetcode.cn/problems/longest-palindromic-substring/description/)

struct Pair {
    start: i32,
    end: i32,
}

fn check(s: &[u8], mut i: i32, mut j: i32, pair: &mut Pair) {
    let n = s.len() as i32;
    while i >= 0 && j <= n - 1 && s[i as usize] == s[j as usize] {
        i -= 1;
        j += 1;
    }
    if j - i - 1 > pair.end - pair.start + 1 {
        pair.start = i + 1;
        pair.end = j - 1;
    }
}

pub fn longest_palindrome(s: String) -> String {
    let s_bytes = s.as_bytes();
    let n = s_bytes.len();
    if n == 0 {
        return s;
    }
    let mut pair = Pair { start: 0, end: 0 };
    for i in 0..n - 1 {
        check(s_bytes, i as i32, i as i32, &mut pair);
        check(s_bytes, i as i32, (i + 1) as i32, &mut pair);
    }
    String::from_utf8(s_bytes[pair.start as usize..=pair.end as usize].to_vec()).unwrap()
}

pub fn longest_palindrome_dp(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let s_bytes = s.as_bytes();
    let (mut start, mut end, n) = (0usize, 0usize, s.len());
    let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for j in 0..n {
        dp[j][j] = true;
        for i in 0..j {
            if s_bytes[i] == s_bytes[j] {
                dp[i][j] = if i == j - 1 { true } else { dp[i + 1][j - 1] };
                if dp[i][j] && j - i > end - start {
                    start = i;
                    end = j;
                }
            }
        }
    }
    String::from_utf8(s_bytes[start..=end].to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindrome_test() {
        let s = String::from("babad");
        let r = longest_palindrome(s);
        println!("{r}");
    }

    #[test]
    fn longest_palindrome_dp_test() {
        let s = String::from("babad");
        let r = longest_palindrome_dp(s);
        println!("{r}");
    }
}
