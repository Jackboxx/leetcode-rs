fn main() {
    println!("{}", find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut vec = vec![];
    let mut i = 0;
    let mut j = 0;

    loop {
        if nums1.get(i).is_none() && nums2.get(j).is_none() {
            break;
        }

        let n1 = nums1.get(i).unwrap_or(&i32::MAX);
        let n2 = nums2.get(j).unwrap_or(&i32::MAX);

        if n1 <= n2 {
            vec.push(n1);
            i += 1;
        } else {
            vec.push(n2);
            j += 1;
        }
    }

    let len = vec.len();

    if len % 2 == 0 {
        let index = len / 2;
        (**vec.get(index).unwrap() + **vec.get(index - 1).unwrap()) as f64 / 2.0
    } else {
        let index = len / 2;
        **vec.get(index).unwrap() as f64
    }
}
