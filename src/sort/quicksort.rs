pub fn quicksort(nums: &mut Vec<i32>) {
    let len = nums.len();
    sort(nums, 0, len - 1);
}

fn sort(nums: &mut Vec<i32>, left: usize, right: usize) {
    let standard = nums[left];
    let (mut i, mut j) = (left, right);

    while i < j {
        while nums[j] >= standard && i < j {
            j -= 1;
        }

        while nums[i] <= standard && i < j {
            i += 1;
        }

        let temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }
    sort(nums, left, j - 1);
    sort(nums, i + 1, right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![3, 2, 1, 5, 4];
        quicksort(&mut nums);
        println!("{:?}", nums);
    }
}
