fn main() {
    let value = two_sum(vec![0,2,4],6);
    println!("{}{}",value[0],value[1]);
}
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (outer_index,outer_value) in nums.iter().enumerate() {
        for (inner_index,inner_value) in nums[1+outer_index..].iter().enumerate() {
            if outer_value + inner_value == target { return vec![outer_index as i32,1+inner_index as i32 + outer_index as i32] }
        }
    }
    return vec![-1,-1];
}