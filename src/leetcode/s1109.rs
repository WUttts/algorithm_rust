///###[1109. 航班预订统计](https://leetcode.cn/problems/corporate-flight-bookings/)
#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let nums = vec![0; n as usize];
        let mut diff = Difference::new(nums);
        for item in bookings {
            diff.increment(item[0] - 1, item[1] - 1, item[2]);
        }
        diff.result()
    }
}

#[allow(unused)]
struct Difference {
    diff: Vec<i32>,
}
#[allow(unused)]
impl Difference {
    fn new(nums: Vec<i32>) -> Self {
        let mut diff = vec![0; nums.len()];
        diff[0] = nums[0];
        for i in 1..diff.len() {
            diff[i] = nums[i - 1] - nums[i];
        }
        Self { diff }
    }

    fn increment(&mut self, i: i32, j: i32, val: i32) {
        let (i, j) = (i as usize, j as usize);
        self.diff[i] += val;
        if j + 1 < self.diff.len() {
            self.diff[j + 1] -= val;
        }
    }

    fn result(&self) -> Vec<i32> {
        let mut res = vec![0; self.diff.len()];
        res[0] = self.diff[0];

        for i in 1..res.len() {
            res[i] = res[i - 1] + self.diff[i];
        }
        res
    }
}
