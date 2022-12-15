use std::collections::HashMap;
///###[76. 最小覆盖子串](https://leetcode.cn/problems/minimum-window-substring/)
/// 滑动窗口
pub fn min_window(s: String, t: String) -> String {
    if s.len() == 0 || t.len() == 0 {
        return "".to_string();
    }
    let mut needs = HashMap::new();
    let mut window = HashMap::new();

    let s = s.chars().collect::<Vec<_>>();

    for ch in t.chars() {
        needs.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let (mut left, mut right) = (0, 0);
    let mut valid = 0;
    let mut start = 0;
    let mut len = usize::MAX;

    while right < s.len() {
        if needs.contains_key(&s[right]) {
            window
                .entry(s[right])
                .and_modify(|cnt| *cnt += 1)
                .or_insert(1);
            if window[&s[right]] == needs[&s[right]] {
                valid += 1;
            }
        }
        right += 1;
        while valid == needs.len() {
            if right - left < len {
                start = left;
                len = right - left;
            }
            if needs.contains_key(&s[left]) {
                if window[&s[left]] == needs[&s[left]] {
                    valid -= 1;
                }
                window.entry(s[left]).and_modify(|cnt| *cnt -= 1);
            }
            left += 1;
        }
    }
    if len == usize::MAX {
        "".to_string()
    } else {
        s[start..start + len].into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("EBBANCF");
        let t = String::from("ABC");

        println!("{}", min_window(s, t));
    }
}
