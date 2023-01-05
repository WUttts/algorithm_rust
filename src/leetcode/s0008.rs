///## https://leetcode.cn/problems/string-to-integer-atoi/
pub fn my_atoi(s: String) -> i32 {
    let mut state = State::Start;
    let mut negtive = false;

    let mut result = 0i64;
    for b in s.bytes() {
        match state {
            State::Start => {
                if b == b' ' {
                    continue;
                } else if b >= b'0' && b <= b'9' {
                    result = result * 10 + (b - b'0') as i64;
                    state = State::FoundNumber;
                } else if b == b'-' || b == b'+' {
                    negtive = b == b'-';
                    state = State::FoundSymbol;
                } else {
                    break;
                }
            }
            State::FoundSymbol | State::FoundNumber => {
                if b >= b'0' && b <= b'9' {
                    result = result * 10 + (b - b'0') as i64;
                    state = State::FoundNumber;
                    if result >= (i32::MAX as i64) + 1 {
                        break;
                    }
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
            0 - (result as i32)
        } else {
            result as i32
        }
    }
}

enum State {
    Start,       // 起始, 包含前导空格
    FoundSymbol, // 已经找到第一个非数字符号
    FoundNumber, // 找到数字
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(123, my_atoi("0000123".to_string()));
    }
}
