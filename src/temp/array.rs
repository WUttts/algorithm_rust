pub fn longest_palindrome(s: String) -> String {
    fn palindrome(s: &String, mut l: i32, mut r: usize) -> String {
        let chars = s.as_bytes();
        while (r < s.len() && l >= 0) && chars[l as usize] == chars[r] {
            l -= 1;
            r += 1;
        }
        s[(l + 1) as usize..r].to_string()
    }

    let mut res = String::from("");
    for i in 0..s.len() {
        let s1 = palindrome(&s, i as i32, i);
        let s2 = palindrome(&s, i as i32, i + 1);
        res = if res.len() < s1.len() { s1 } else { res };
        res = if res.len() < s2.len() { s2 } else { res };
    }
    res
}
