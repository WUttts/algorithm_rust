///[整数反转](https://leetcode.cn/problems/reverse-integer/)
/// ```
///    input x = 123
///    output x = 321
/// ```
/// i32 Max = 2147483647
/// i32 Min = -2147483648
pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let max_flag = i32::MAX / 10;
    let min_flag = i32::MIN / 10;
    let mut res = 0;
    while x != 0 {
        let tmp = x % 10;
        //判断值过大越界的情况
        if res > max_flag || (res == max_flag && tmp > 7) {
            println!("1");
            return 0;
        }
        //判断值过小越界的情况
        if res < min_flag || (res == min_flag && tmp < -8) {
            println!("2");

            return 0;
        }
        res = res * 10 + tmp;
        x /= 10;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(321,reverse(123));
    }

    #[test]
    fn test_2() {
        assert_eq!(0,reverse(1147483649));
    }

    #[test]
    fn test_3() {
        assert_eq!(0,reverse(-1147483649));
    }

    #[test]
    fn test_4() {
        assert_eq!(1,reverse(10));
    }
}
