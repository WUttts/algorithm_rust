/**
 * 差分数组，原题是370.区间加法，由于需要会员，故在此记录问题。
 * 假设有一长度为n数组，初始状态下元素皆为0，nums=[0,0,0,0,0];
 * 现在要对该数组进行K个更新操作，
 * 输入操作：[[1,3,2],[0,2,1],[2,4,-2]];
 * 其中[1,3,2]表示对nums[1..=3]进行加2的操作
 */

///## 解答
/// 差分数组
struct Difference {
    diff: Vec<i32>,
}
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

pub fn get_modified_array(len: i32, update: Vec<Vec<i32>>) -> Vec<i32> {
    let nums = vec![0; len as usize];
    let mut diff = Difference::new(nums);
    for item in update {
        diff.increment(item[0], item[1], item[2]);
    }
    diff.result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_modified_array() {
        let update = vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]];

        let res = get_modified_array(5, update);
        assert_eq!(res, vec![-2, 0, 3, 5, 3]);
    }
}
