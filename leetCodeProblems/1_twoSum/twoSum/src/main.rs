use std::collections::HashMap;
fn main() {
    let value = two_sum(vec![3,3],6);
    println!("{}{}",value[0],value[1]);
}
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //double loop version
    /*
    for (outer_index,outer_value) in nums.iter().enumerate() {
        for (inner_index,inner_value) in nums[1+outer_index..].iter().enumerate() {
            if outer_value + inner_value == target { return vec![outer_index as i32,1+inner_index as i32 + outer_index as i32] }
        }
    }
    */
    //now lets do the has table version for extra speed at the cost of some memory
    let mut value_index_map = HashMap::new();
    //loop through the input
    for (index,value) in nums.iter().enumerate() {
        let complement = target - *value;
        //check to see if complement already exists in the hash
        //the first 2 times this runs it will not have anything to compare
        //this is where the monad Some is awesome!
        if let Some(complement_in_hash) = value_index_map.get(&complement)
        {
            return vec![*complement_in_hash,index as i32];   
        }
        value_index_map.insert(*value,index as i32);
    }
    return vec![];
}