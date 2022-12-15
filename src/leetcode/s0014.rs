///## https://leetcode.cn/problems/longest-common-prefix/
/// 最长公共前缀
///
/// 输入：strs = \["flower","flow","flight"\]
///
/// 输出："fl"
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut result = strs[0].clone();
    for s in strs {
        let mut j = 0;
        let mut temp = String::from("");
        while j < result.len() && j < s.len() {
            if result.as_bytes()[j] != s.as_bytes()[j] {
                break;
            }
            temp.push(result.as_bytes()[j] as char);
            j += 1;
        }
        result = temp;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(String::from("fl"), longest_common_prefix(strs));
    }
    #[test]
    fn test_2() {
        let strs = vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ];
        assert_eq!(String::from(""), longest_common_prefix(strs));
    }
}
