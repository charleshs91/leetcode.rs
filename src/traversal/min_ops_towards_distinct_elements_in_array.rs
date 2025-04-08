/**
 * 3396. Minimum Number of Operations to Make Elements in Array Distinct
 * 
 * https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct
 */
pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut seen = [false; 128];
    for i in (0..nums.len()).rev() {
        let num = nums[i] as usize;
        if seen[num] {
            return (i as i32) / 3 + 1;
        }
        seen[num] = true
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_operations_case1() {
        assert_eq!(minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]), 2)
    }

    #[test]
    fn test_minimum_operations_case2() {
        assert_eq!(minimum_operations(vec![4, 5, 6, 4, 4]), 2)
    }

    #[test]
    fn test_minimum_operations_case3() {
        assert_eq!(minimum_operations(vec![6, 7, 8, 9]), 0)
    }
}