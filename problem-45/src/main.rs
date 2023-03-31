fn main() {
    println!("{}", jump(vec![3, 4, 3, 2, 5, 4, 3]));
}

fn jump(nums: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut count = 0;

    loop {
        if index >= nums.len() - 1 {
            return count;
        }

        let max_steps = *nums.get(index).unwrap();

        if index + max_steps as usize >= nums.len() - 1 {
            return count + 1;
        }

        let best = nums
            .iter()
            .skip(index + 1)
            .take(max_steps as usize)
            .enumerate()
            .map(|(i, x)| {
                (
                    i,
                    (nums.len() - 1).checked_sub(*x as usize + i).unwrap_or(0),
                )
            })
            .min_by(|(_, a), (_, b)| a.cmp(&b))
            .map(|(i, _)| i)
            .unwrap()
            + 1;

        index += best as usize;
        count += 1;
    }
}
