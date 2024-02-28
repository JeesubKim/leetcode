impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{

        // easiest way will be O(n^2)
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j continue;
                if nums[i] + nums[j] == target {
                    vec![i as i32, j as i32]
                }
            }
        }
        Vec::new()
    }
}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //in case that nums are in order
        let mut left:i32 = 0;
        let mut right:i32 = nums.len() as i32 - 1;

        while left < right {
            let sum = nums[left as usize] + nums[right as usize];
            if sum == target {
                break;
            } else if sum > target {
                right -= 1;
            } else {
                left +=1;
            }
        }

        vec![left,right]
    }
}

use std::collections::HashMap;
impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Using Hash, we can get the value in O(n)
        //
        let mut hashmap = HashMap::new();

        for (index, element) in numbers.iter().enumerate() {
            let val = target - element;

            if hashmap.contains_key(&val) {
                return vec![hashmap[&val], index as i32];
            }

            hashmap.insert(element, index as i32);
        }

    }

}
