fn main() {
    let mut vec = vec![1, 1, 1, 2, 2, 3];
    println!("{}", remove_duplicates(&mut vec));
    println!("{:?}", vec);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut count = 0;
    let mut index = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count += 1;
        } else {
            count = 0;
        }

        if count <= 1 {
            nums[index] = nums[i];
            index += 1;
        }
    }

    index as i32
}
