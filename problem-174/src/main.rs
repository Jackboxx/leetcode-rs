fn main() {
    println!(
        "{}",
        calculate_minimum_hp(vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]])
    );
    println!("{}", calculate_minimum_hp(vec![vec![-3, 5]]));
    println!("{}", calculate_minimum_hp(vec![vec![-200]]));
}

fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let n = dungeon.len();
    let m = dungeon.get(0).unwrap().len();

    let mut cost_vec = vec![vec![0; m]; n];
    cost_vec[n - 1][m - 1] = dungeon[n - 1][m - 1].min(0);

    for i in (0..m - 1).rev() {
        cost_vec[n - 1][i] = (cost_vec[n - 1][i + 1] + dungeon[n - 1][i]).min(0);
    }

    for i in (0..n - 1).rev() {
        cost_vec[i][m - 1] = (cost_vec[i + 1][m - 1] + dungeon[i][m - 1]).min(0);
    }

    for y in (0..n - 1).rev() {
        for x in (0..m - 1).rev() {
            let right = cost_vec[y][x + 1];
            let bottom = cost_vec[y + 1][x];

            cost_vec[y][x] = (i32::max(right, bottom) + dungeon[y][x]).min(0);
        }
    }

    cost_vec[0][0].abs() + 1
}
