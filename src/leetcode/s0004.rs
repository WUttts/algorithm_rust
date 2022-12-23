///## 寻找中位数
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mid = (nums1.len() + nums2.len()) / 2;
    let mut cnt = 0;
    let (mut i, mut j) = (0, 0);
    while cnt < mid {
        cnt += 1;
        if nums1[i] <= nums2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    println!("{},{}", i, j);
    2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 4, 5];
        let nums2 = vec![1, 2, 3, 4];

        assert_eq!(2.5f64, find_median_sorted_arrays(nums1, nums2));
    }
    #[test]
    fn test_2() {
        let nums1 = vec![1, 3, 4, 9];
        let nums2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(2.0f64, find_median_sorted_arrays(nums1, nums2));
    }
}
