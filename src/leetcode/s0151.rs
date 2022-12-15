/// [反转字符串中的单词](https://leetcode.cn/submissions/detail/379627076/)
/// 要求原地修改
/// 写了个又臭又长的方法,主要是去掉多余空格那稍嫌啰嗦了
/// 思路是，先反转整个字符串，再反转单个单词。
/// 然后去掉多余空格
#[allow(unused)]
pub fn reverse_words(s: String) -> String {
    let mut s = s;
    let reverse = |bytes: &mut [u8], mut left: usize, mut right: usize| {
        while left < right {
            let temp = bytes[left];
            bytes[left] = bytes[right];
            bytes[right] = temp;

            left += 1;
            right -= 1;
        }
    };
    unsafe {
        let bytes = s.as_bytes_mut();
        reverse(bytes, 0, bytes.len() - 1);
        let mut i = 0usize;
        let mut j = 0usize;

        while i < bytes.len() {
            if bytes[i] == 32u8 {
                i += 1;
                continue;
            }
            j = i;
            while j < bytes.len() && bytes[j] != 32u8 {
                j += 1;
            }

            reverse(bytes, i, j - 1);
            i = j + 1;
        }
        //去除多余空格
        i = 0;
        j = 0;
        let mut k = 0;
        while j < bytes.len() {
            if bytes[j] == 32u8 {
                j += 1;
                continue;
            }
            k = j;
            while k < bytes.len() && bytes[k] != 32u8 {
                k += 1;
            }
            while j < k {
                bytes[i] = bytes[j];
                j += 1;
                i += 1;
            }
            if i < bytes.len() {
                bytes[i] = 32u8;
                i += 1;
            }
        }
        s = if i < bytes.len() && (bytes[i] == 32u8 || bytes[i - 1] == 32u8) {
            s[..i - 1].to_string()
        } else if bytes[i - 1] == 32u8 {
            s[..i - 1].to_string()
        } else {
            s[..i].to_string()
        }
    }
    s
}

/// 其实用API的话会更快
pub fn reverse_words_2(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
