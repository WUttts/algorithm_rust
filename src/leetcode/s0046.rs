pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];

    dfs(&mut nums, &mut ans, 0);
    ans
}

fn dfs(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, begin: usize) {
    if begin == nums.len() {
        println!("{:?}", nums);
        ans.push(nums.to_vec());
        return;
    }

    for i in begin..nums.len() {
        nums.swap(i, begin);
        dfs(nums, ans, begin + 1);
        nums.swap(i, begin);
    }
}

#[cfg(test)]
mod tests {
    use super::permute;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        let res = permute(nums);
        println!("{:?}", res);
    }
}
