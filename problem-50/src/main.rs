fn main() {
    println!("{}", 10.0_f64.powi(5));
    println!("{}", 10.0_f64.powi(-5));
    println!("{}", 0.44528_f64.powi(0));
    println!("{}", 0.00001_f64.powi(2147483647));
    println!("{}", 2.0_f64.powi(-2147483647));
}

fn my_pow(x: f64, n: i32) -> f64 {
    x.powi(n)
}
