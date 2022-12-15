///#303. 区域和检索 - 数组不可变
/// https://leetcode.cn/problems/range-sum-query-immutable/
///
#[allow(unused)]
struct NumArray {
    nums: Vec<i32>,
}
#[allow(unused)]
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut vec = vec![0; nums.len() + 1];
        for i in 1..=nums.len() {
            vec[i] = vec[i - 1] + nums[i - 1];
        }

        Self { nums: vec }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.nums[(right + 1) as usize] - self.nums[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let mut res = vec![];
        let num_array = NumArray::new(nums);
        res.push(num_array.sum_range(0, 2));
        res.push(num_array.sum_range(2, 5));
        res.push(num_array.sum_range(0, 5));

        println!("{:?}", res);
    }
}
