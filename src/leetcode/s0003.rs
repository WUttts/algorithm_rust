use std::collections::{HashMap, HashSet};

///无重复最长子串
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut left = 0;
    let mut cur_len = 0;
    let mut res = 0;
    let mut set = HashSet::new();
    let bytes = s.as_bytes();

    for i in 0..s.len() {
        cur_len += 1;
        while set.contains(&bytes[i]) {
            set.remove(&bytes[left]);
            left += 1;
        }
        res = res.max(cur_len);
        set.insert(bytes[i]);
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abcabcbb");

        println!("{}", length_of_longest_substring(s));
    }
    #[test]
    fn test_1() {
        let s = String::from("abcabcbb");
        assert_eq!(3, length_of_longest_substring(s));
    }
    #[test]
    fn test_2() {
        let s = String::from("bbbbbbbbbbbb");
        assert_eq!(1, length_of_longest_substring(s));
    }
    #[test]
    fn test_3() {
        let s = String::from("pwwkew");
        assert_eq!(3, length_of_longest_substring(s));
    }
    #[test]
    fn test_4() {
        let s = String::from(" ");
        assert_eq!(1, length_of_longest_substring(s));
    }
    #[test]
    fn test_5() {
        let s = String::from("au");
        assert_eq!(2, length_of_longest_substring(s));
    }
}
