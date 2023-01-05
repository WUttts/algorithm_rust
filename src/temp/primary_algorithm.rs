use std::collections::{HashMap, HashSet};

enum State {
    Start,
    Number,
    Symbol,
}

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

/// ## 旋转数组
/// 使用临时数组，思考坐标与轮转步数的关系
pub fn rotate_1(nums: &mut Vec<i32>, k: i32) {
    //index = (i + k) % nums.len();
    let k = k as usize;
    let nums_tmp = nums.clone();
    for i in 0..nums_tmp.len() {
        nums[(i + k) % nums_tmp.len()] = nums_tmp[i];
    }
}

///## 多次反转
pub fn rotate_2(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize;
    let reverse = |nums: &mut Vec<i32>, mut start: usize, mut end: usize| {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    };
    //先全部反转
    reverse(nums, 0, nums.len() - 1);
    //反转前K个
    reverse(nums, 0, k - 1);
    //反转后k个
    reverse(nums, k, nums.len() - 1);
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for i in nums {
        if set.contains(&i) {
            return true;
        }
        set.insert(i);
    }
    return false;
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut reduce = 0;
    for i in nums {
        reduce = reduce ^ i;
    }

    reduce
}

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (mut nums1, mut nums2) = (nums1, nums2);
    let (mut i, mut j) = (0, 0);
    let mut res = vec![];
    nums1.sort();
    nums2.sort();
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            i += 1;
        } else if nums2[j] < nums1[i] {
            j += 1;
        } else {
            res.push(nums1[i]);
            i += 1;
            j += 1;
        }
    }

    res
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut len = digits.len() - 1;
    digits[len] += 1;
    while digits[len] >= 10 && len > 0 {
        digits[len] %= 10;
        len -= 1;
        digits[len] += 1;
    }
    if digits[0] >= 10 {
        digits.insert(1, digits[0] % 10);
        digits[0] = 1;
    }
    digits
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if nums[j] != 0 {
            nums.swap(i, j);
            i += 1;
        }
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if map.contains_key(&(target - nums[i])) {
            return vec![map[&(target - nums[i])] as i32, i as i32];
        }
        map.insert(nums[i], i);
    }
    vec![]
}

pub fn reverse_string(s: &mut Vec<char>) {
    let (mut i, mut j) = (0, s.len() - 1);
    while i < j {
        let tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;
        i += 1;
        j -= 1;
    }
}

pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut rev = 0;
    while x != 0 {
        let rest = x % 10;
        if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && rest > 7) {
            return 0;
        }
        if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && rest < -8) {
            return 0;
        }
        rev = rev * 10 + rest;
        x /= 10;
    }
    rev
}

pub fn first_uniq_char(s: String) -> i32 {
    let mut count = vec![0; 26];
    let s = s.as_bytes();
    for b in s {
        count[(*b - b'a') as usize] += 1;
    }
    for (i, b) in s.iter().enumerate() {
        if count[(*b - b'a') as usize] == 1 {
            return i as i32;
        }
    }
    -1
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut counts = [0; 26];
    for i in 0..s.len() {
        counts[(s[i] - b'a') as usize] += 1;
        counts[(t[i] - b'a') as usize] -= 1;
    }
    for i in counts {
        if i != 0 {
            return false;
        }
    }
    true
}

pub fn is_palindrome(s: String) -> bool {
    let chars = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<_>>();
    let (mut start, mut end) = (0, chars.len() - 1);
    while start < end {
        if chars[start] != chars[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

pub fn my_atoi(s: String) -> i32 {
    let mut result = 0i64;
    let mut negtive = false;
    let mut state = State::Start;
    for b in s.bytes() {
        match state {
            State::Start => {
                if b == b' ' {
                    continue;
                } else if b >= b'0' && b <= b'9' {
                    result = result * 10 + (b - b'0') as i64;
                    state = State::Number;
                } else if b == b'-' || b == b'+' {
                    negtive = b == b'-';
                    state = State::Symbol;
                } else {
                    break;
                }
            }
            State::Number | State::Symbol => {
                if b >= b'0' && b <= b'9' {
                    result = result * 10 + (b - b'0') as i64;
                    if result > (i32::MAX) as i64 {
                        break;
                    }
                    state = State::Number;
                } else {
                    break;
                }
            }
        }
    }
    if result >= (i32::MAX as i64) + 1 {
        if negtive {
            i32::MIN
        } else {
            i32::MAX
        }
    } else {
        if negtive {
            -result as i32
        } else {
            result as i32
        }
    }
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut i = 0;
    let mut j = 0;

    while i < haystack.len() && j < needle.len() {
        if haystack.as_bytes()[i] == needle.as_bytes()[j] {
            i += 1;
            j += 1;
        } else {
            i = i - j + 1;
            j = 0;
        }
        if j == needle.len() {
            return (i - j) as i32;
        }
    }
    -1
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut res = i32::MIN;
    let mut dp = vec![0; len];
    dp[0] = nums[0];
    for i in 1..len {
        if dp[i - 1] > 0 {
            dp[i] = dp[i - 1] + nums[i];
        } else {
            dp[i] = nums[i];
        }
    }
    for i in dp {
        res = res.max(i);
    }
    res
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = nums1.len() - 1;
    let mut n = n - 1;
    let mut m = m - 1;
    while n >= 0 {
        while m >= 0 && nums1[m as usize] > nums2[n as usize] {
            nums1.swap(m as usize, i);
            i -= 1;
            m -= 1;
        }
        nums1[i] = nums2[n as usize];
        i -= 1;
        n -= 1;
    }
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut res = 0;
    let mut min = prices[0];
    for i in prices {
        min = min.min(i);
        res = res.max(i - min);
    }
    res
}
pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let (r, c) = (r as usize, c as usize);
    let (m, n) = (mat.len(), mat[0].len());
    if r * c != m * n {
        return mat;
    }
    let mut res = vec![vec![0; c]; r];
    for i in 0..r * c {
        res[i / c][i % c] = mat[i / n][i % n];
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotate_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate_1(&mut nums, k);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }
    #[test]
    fn test_rotate_2() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate_2(&mut nums, k);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }
    #[test]
    fn test_plus_one() {
        let digits = vec![9, 9, 9, 9, 9];
        assert!(plus_one(digits) == vec![1, 0, 0, 0, 0, 0]);
    }
    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

        assert_eq!(max_sub_array(nums), 6);
    }

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_matrix_reshape() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let res = matrix_reshape(mat, 1, 4);
        println!("{:?}", res);
    }
}
