#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len() - 1;
        search_recursive(nums, target, 0, len)
    }
}

pub fn search_recursive(nums: Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
    let size = end - start + 1;
    if size == 1 {
        return if nums[start] == target {
            start as i32
        } else {
            -1
        };
    }
    if size == 2 {
        return if nums[start] == target {
            start as i32
        } else if nums[end] == target {
            end as i32
        } else {
            -1
        };
    }

    let middle = (start + end + 1) / 2;
    if nums[middle] == target {
        return middle as i32;
    }

    return if nums[middle] > nums[start] {
        if target < nums[middle] && target >= nums[start] {
            search_recursive(nums, target, start, middle - 1)
        } else {
            search_recursive(nums, target, middle + 1, end)
        }
    } else {
        if target > nums[middle] && target <= nums[end] {
            search_recursive(nums, target, middle + 1, end)
        } else {
            search_recursive(nums, target, start, middle - 1)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn example02() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn example03() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn example04() {
        assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
    }
}
