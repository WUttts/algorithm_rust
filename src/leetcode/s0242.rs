///给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
///注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut alpha = [0; 26];

    for i in 0..s.len() {
        alpha[(s[i] - 97) as usize] += 1;
        alpha[(t[i] - 97) as usize] -= 1;
    }

    for i in alpha {
        if i != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert_eq!(true, is_anagram(s, t));
    }
}
