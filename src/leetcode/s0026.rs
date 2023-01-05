pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut slow, mut fast) = (0, 0);
    while fast < nums.len() {
        if nums[slow] != nums[fast] {
            slow += 1;
            nums[slow] = nums[fast];
        }
        fast += 1;
    }

    (slow + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, remove_duplicates(&mut nums));
    }
    #[test]
    fn test_2() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates(&mut nums));
    }
}
