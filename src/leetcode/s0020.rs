use std::collections::HashMap;

/// ({[]})
/// () {} []
/// ([)]
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec!['0'];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        return false;
                    }
                }
                _ => (),
            }
        }
        stack.len() == 1
    }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let s = "{[()]}".to_string();
        assert!(is_valid(s));
    }
    #[test]
    fn test_2() {
        let s = "(){}[]".to_string();
        assert_eq!(is_valid(s), true);
    }
    #[test]
    fn test_3() {
        let s = "([)]".to_string();
        assert_eq!(is_valid(s), false);
    }
}
