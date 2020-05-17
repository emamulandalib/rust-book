use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut comp = HashMap::new();

    for i in 0..nums.len() {
        match comp.get(&nums[i]) {
            Some(v) => {
                if *v >= 0 {
                    return vec![*v, i as i32]
                }
            }
            _ => {}
        }

        comp.insert(target - nums[i], i as i32);
    }

    return vec![];
}

fn main() {
    let ts = two_sum(vec![1, 2, 3, 5], 4);
    println!("{:?}", ts)
}
