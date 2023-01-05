use std::cmp::Ordering;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut slow = 0;

    for fast in 0..nums.len() {
        if nums[fast] != nums[slow] {
            slow += 1;
            nums[slow] = nums[fast];
        }
    }
    (slow + 1) as i32
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut slow = 0;
    for fast in 0..nums.len() {
        if nums[fast] != val {
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    slow as i32
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut slow = 0;
    for fast in 0..nums.len() {
        if nums[fast] != 0 {
            let temp = nums[slow];
            nums[slow] = nums[fast];
            nums[fast] = temp;
            slow += 1;
        }
    }
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut l, mut r) = (0, numbers.len() - 1);
    while l <= r {
        match (numbers[l] + numbers[r]).cmp(&target) {
            Ordering::Equal => return vec![l as i32, r as i32],
            Ordering::Greater => r -= 1,
            Ordering::Less => l += 1,
        }
    }
    vec![]
}
pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    let mut temp;
    for i in 0..len / 2 {
        temp = s[i];
        s[i] = s[len - i - 1];
        s[len - i - 1] = temp;
    }
}
pub fn longest_palindrome(s: String) -> String {
    fn palindrome(s: &String, mut l: i32, mut r: usize) -> String {
        let chars = s.as_bytes();
        while (l >= 0 && r < s.len()) && chars[l as usize] == chars[r] {
            l -= 1;
            r += 1;
        }
        s[(l + 1) as usize..r].to_string()
    }

    let mut res = String::from("");
    for i in 0..s.len() {
        let s1 = palindrome(&s, i as i32, i);
        let s2 = palindrome(&s, i as i32, i + 1);
        res = if res.len() >= s1.len() { res } else { s1 };
        res = if res.len() >= s2.len() { res } else { s2 };
    }
    res
}

///# 该方法作为指导，所以需要传入排序好的数组
///两数之和,试想,如果需要返回nums里的所有满足关系的元素，且不能重复，如何做
/// nums = [1,3,1,2,2,3], target = 4
/// 那么算法返回的结果就是：[[1,3],[2,2]]。比如上述例子中 [1,3] 和 [3,1] 就算重复，只能算一次。
pub fn two_sum_batch(nums: &Vec<i32>, mut left: usize, target: i64) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut res = vec![];
    if len < 2 {
        return res;
    }
    //因为已经排好序了，所以可以先判断，避免没必要的计算
    if nums[0] as i64 + nums[1] as i64 > target {
        return res;
    }
    let mut right = len - 1;

    while left < right {
        match nums[left] as i64 + nums[right] as i64 - target as i64 {
            0 => {
                //添加结果
                // 所以nums[left]，nums[right]没必要再用了，直接跳过。
                res.push(vec![nums[left], nums[right]]);
                //target = a + b = b + a,
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                left += 1;
                right -= 1;
            }
            a if a > 0 => {
                //a + b > target,a<b,所以也就是b不满足关系，如果前一个的值相同，那么就没必要继续下去了
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                //此时指向最后一个b,它的前一个满足该式，如：1,1,1,2,2,3,3,4,4,4 target = 4
                //经过上面的过滤,right此时指向第一个4，所以需要再减一.
                right -= 1;
            }

            _ => {
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                left += 1;
            }
        }
    }

    res
}

///看上一个方法two_sum_batch
///输入nums,target 返回满足关系的不重复两两数对。
/// 那么三数之和，target = nums[i]+nums[j]+nums[k];
/// target-nums[i] = nums[j]+nums[k];
/// target-nums[i] 就是两数之和的新target.
pub fn three_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let len = nums.len();
    let mut res = vec![];
    if len < 3 {
        return res;
    }
    nums.sort();
    if nums[0] + nums[1] + nums[2] > target {
        return res;
    }

    for i in 0..len {
        //首先第一个元素比较后，有可能满足条件或不满足，但总的来说，这个数"已经没用了"
        //而后续的数仍然可能是相等的，所以可以先去重
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        //此时，就是两数之和了
        let two_sum_res = two_sum_batch(&nums, i + 1, target as i64 - nums[i] as i64);
        for mut item in two_sum_res {
            item.push(nums[i]);
            res.push(item);
        }
    }
    res
}

///四数之和 = target = nums[i]+nums[j]+nums[k]+nums[l];
/// target - nums[i] = nums[j]+nums[k]+nums[l];
/// 所以target - nums[i] 需要和另外三个数满足关系，这又是一个三数之和。
///
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut res = vec![];
    let len = nums.len();
    if len < 4 {
        return res;
    }
    nums.sort();
    if nums[0] as i64 + nums[1] as i64 + nums[2] as i64 + nums[3] as i64 > target as i64 {
        return res;
    }

    for i in 0..len {
        //当第一个元素满足或不满足条件后，去重
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..len {
            //当第二个元素满足或不满足条件后，去重
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            let two_sum_res = two_sum_batch(
                &nums,
                j + 1,
                target as i64 - nums[i] as i64 - nums[j] as i64,
            );
            for mut item in two_sum_res {
                item.push(nums[j]);
                item.push(nums[i]);
                res.push(item);
            }
        }
    }
    res
}

///前缀和例子
/// 给定一个数组，求该数组的区间和
/// 输入：nums[1,2,3,4,5] left=2,right=4
/// 输出：6
/// 解释：2 + 4 = 6
/// 这是一道痕简单的题，但要思考一个问题如果经常调用这个方法，就会有性能上的影响
/// 所以有一种很巧妙的方法，把O(N)的时间复杂度度变为O(1),具体细节可以去看看,这里只做记录
/// [303.区域检索-数组不可变](https://leetcode.cn/problems/range-sum-query-immutable/)
pub fn preix_add() {
    fn new(nums: Vec<i32>) -> Vec<i32> {
        let mut pre_sum = vec![0; nums.len() + 1];
        for i in 1..pre_sum.len() {
            pre_sum[i] = pre_sum[i - 1] + nums[i - 1];
        }
        pre_sum
    }

    //求区间和
    fn between(nums: &Vec<i32>, left: usize, right: usize) -> i32 {
        return nums[right + 1] - nums[left];
    }
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    let pre_sum = new(nums);
    //结果应是10
    let res = between(&pre_sum, 0, 3);
    println!("{res}");
}

/// 差分数组
#[allow(unused)]
pub fn difference() {
    let nums = vec![8, 2, 6, 3, 1];
    //构造差分数组
    fn diff(nums: Vec<i32>) -> Vec<i32> {
        let mut sub = vec![0; nums.len()];
        sub[0] = nums[0];
        for i in 1..nums.len() {
            sub[i] = nums[i] - nums[i - 1];
        }
        sub
    }
    //这样可还原数组
    fn _recover(diff: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; diff.len()];
        res[0] = diff[0];
        for i in 1..diff.len() {
            res[i] = res[i - 1] + diff[i];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }
    #[test]
    fn test_reverse_string() {
        let mut s = vec!['\"'];
        reverse_string(&mut s);
    }

    #[test]
    fn test_longest_palindrome() {
        let s = "babad".to_string();
        let res = longest_palindrome(s);
        println!("{res}");
        assert_eq!(res, String::from("bab"));
    }

    #[test]
    fn test_two_sum_batch() {
        let nums = vec![-2, 0, 1, 1, 2];
        let res = two_sum_batch(&nums, 0, 2);
        println!("{:?}", res);
        assert!(res.len() > 0);
        for item in res {
            assert_eq!(item[0] + item[1], 2);
        }
    }

    #[test]
    fn test_three_sum() {
        let nums = vec![-2, 0, 1, 1, 2];
        let res = three_sum(nums, 0);
        println!("{:?}", res);
    }

    #[test]
    fn test_four_sum() {
        let nums = vec![
            -1000000000,
            -1000000000,
            1000000000,
            -1000000000,
            -1000000000,
        ];

        let res = four_sum(nums, 294967296);
        println!("{:?}", res);
    }
    #[test]
    fn test_preix_add() {
        preix_add();
    }
}
