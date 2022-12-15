///## https://leetcode.cn/problems/container-with-most-water/
/// 在每个状态下，无论长板或短板向中间收窄一格，都会导致水槽 底边宽度 -1变短：
/// - 若向内 移动短板 ，水槽的短板 min(h[i], h[j])可能变大，因此下个水槽的面积 可能增大 。
/// - 若向内 移动长板 ，水槽的短板 min(h[i], h[j])不变或变小，因此下个水槽的面积 一定变小 。
/// 
/// 因此，初始化双指针分列水槽左右两端，循环每轮将短板向内移动一格，并更新面积最大值，直到两指针相遇时跳出；
/// 即可获得最大面积。
pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut i, mut j) = (0, height.len() - 1);
    let mut res = 0;
    while i < j {
        if height[i] < height[j] {
            res = res.max(height[i] * (j - i) as i32);
            i += 1;
        } else {
            res = res.max(height[j] * (j - i) as i32);
            j -= 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(49, max_area(height));
    }
}
