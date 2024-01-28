#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }

        let mut start: i32 = 0;
        let mut finish: i32 = nums.len() as i32 - 1;

        loop {
            println!("start: {}, finish: {}", start, finish);
            if start > finish {
                break;
            }
            let middle_index = (start + finish) / 2;
            let middle_value = nums[middle_index as usize];

            if middle_value == target {
                return middle_index as i32;
            }

            if middle_value > target {
                finish = middle_index - 1;
            } else {
                start = middle_index + 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn example02() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn example03() {
        let nums = vec![2, 5];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
