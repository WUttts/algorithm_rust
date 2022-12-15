use std::collections::HashMap;

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if s.is_empty() || p.is_empty() {
        return vec![];
    }
    let mut res = vec![];

    let mut needs = HashMap::new();
    for c in p.as_bytes() {
        needs.entry(*c).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut window = HashMap::new();
    let chars = s.as_bytes();
    let mut left = 0;
    let mut right = 0;
    let mut valid = 0;

    while right < chars.len() {
        if needs.contains_key(&chars[right]) {
            window
                .entry(chars[right])
                .and_modify(|v| *v += 1)
                .or_insert(1);

            if window[&chars[right]] == needs[&chars[right]] {
                valid += 1;
            }
        }

        right += 1;
        while right - left >= needs.len() {
            if valid == needs.len() {
                res.push(left as i32);
            }
            if needs.contains_key(&chars[left]) {
                if window[&chars[left]] == needs[&chars[left]] {
                    valid -= 1;
                }
                window.entry(chars[left]).and_modify(|v| *v -= 1);
            }
            left += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("baa");
        let p = String::from("aa");
        let res = find_anagrams(s, p);

        println!("{:?}", res);
    }
}
