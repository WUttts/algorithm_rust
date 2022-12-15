use std::collections::HashMap;

///无重复最长子串
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }
    let s = s.chars().collect::<Vec<_>>();
    let mut window = HashMap::new();

    // let mut max = 0;
    let (mut left, mut right) = (0, 0);

    while right < s.len() {
        window
            .entry(s[right])
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
        if window[&s[right]] > 1 {
            left += 1;
            window.entry(s[right]).and_modify(|cnt| *cnt -= 1);
            if left < right && s[left] == s[right] {
                left += 1;
            }
        }
        right += 1;
    }
    // max = if right == s.len() {
    //     right - left - 1
    // } else {
    //     right - left
    // };
    // max as i32
    (right - left) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abcabcbb");
        // let s = String::from("bbbbbbbbbbbb");
        // let s = String::from("pwwkew");
        // let s = String::from(" ");
        // let s = String::from("au");
        // let s = String::from("ckilbkd");

        println!("{}", length_of_longest_substring(s));
    }
}
