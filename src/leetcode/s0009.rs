///# https://leetcode.cn/problems/palindrome-number/
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut num = x;
    let mut res = 0;

    while num != 0 {
        res = res * 10 + num % 10;
        num /= 10;
    }
    x == res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(true,is_palindrome(121));
    }
}
