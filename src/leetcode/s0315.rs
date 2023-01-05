///315，暴力法会超时
#[allow(unused)]
pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    if len < 2 {
        return vec![0];
    }
    let mut res = vec![0; len];

    let (mut i, mut j) = (0, 0);
    while i < len - 1 {
        while nums[i] >= nums[i + 1] {
            i += 1;
        }
    }
    res
}
