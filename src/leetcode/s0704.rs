pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);
    if nums.len() < 1 || nums[0] > target {
        return 0;
    } else if nums[right] < target {
        return nums.len() as i32;
    }
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}
