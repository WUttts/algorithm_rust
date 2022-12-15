use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.is_empty() || s2.is_empty() {
        return false;
    }
    let mut needs = HashMap::new();
    let mut window = HashMap::new();

    for c in s1.as_bytes() {
        needs.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    let chars = s2.as_bytes();
    let mut left = 0;
    let mut right = 0;
    let mut vaild = 0;
    while right < chars.len() {
        if needs.contains_key(&chars[right]) {
            window
                .entry(chars[right])
                .and_modify(|v| *v += 1)
                .or_insert(1);
            if window[&chars[right]] == needs[&chars[right]] {
                vaild += 1;
            }
        }

        right += 1;

        while right - left >= s1.len() {
            if vaild == needs.len() {
                return true;
            }
            if needs.contains_key(&chars[left]) {
                if window[&chars[left]] == needs[&chars[left]] {
                    vaild -= 1;
                }
                window.entry(chars[left]).and_modify(|v| *v -= 1);
            }
            left += 1;
        }
    }
    false
}

