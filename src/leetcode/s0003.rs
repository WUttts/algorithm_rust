use std::collections::HashSet;

///无重复最长子串
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let bytes = s.as_bytes();

    //记录起始位置和窗口长度。
    let mut left = 0;
    let mut cur_len = 0;
    // 记录最大长度
    let mut res = 0;
    //用于判断是否包含重复字符串
    let mut set = HashSet::new();
    
    for i in 0..s.len() {
        //每遍历一个字符串，窗口长度 +1
        cur_len += 1;
        //如果出现重复字符，则需要缩小窗口
        while set.contains(&bytes[i]) {
            //删掉最左边的值，缩小窗口
            set.remove(&bytes[left]);
            //起始位置 +1
            left += 1;
            //窗口长度 -1
            cur_len -= 1;
        }
        //更新当前的最长窗口
        res = res.max(cur_len);

        set.insert(bytes[i]);
    }

    res
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
