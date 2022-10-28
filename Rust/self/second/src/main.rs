// Need to progress more in Rust to continue this
fn main() {
    println!("Hello, world! Sine 5 is {}", sine(5));
}

/** Computes sine
 *  @param x the turning amount, in degrees
 */
fn sine(x: i128) -> i128 {
    let mut fin_value: i128 = 0;
    let mut temp_add;
    let minus_one: i128 = -1;
    for i in 0..21 {
        temp_add = minus_one.pow(i) * x.pow(2*i + 1);
        temp_add /= factorial((2*i + 1) as usize) as i128;
        fin_value += temp_add;
    }

    return fin_value;
}

fn factorial(x: usize) -> usize {
    if x == 1 {1} else {x * factorial(x-1)}
}

