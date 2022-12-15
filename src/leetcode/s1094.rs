///###[1094. 拼车](https://leetcode.cn/problems/car-pooling/)
#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let nums = vec![0; 1001];
        let mut diff = Difference::new(nums);
        for trip in trips {
            diff.increment(trip[1], trip[2] - 1, trip[0]);
        }

        for i in diff.result() {
            if i > capacity {
                return false;
            }
        }
        true
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
