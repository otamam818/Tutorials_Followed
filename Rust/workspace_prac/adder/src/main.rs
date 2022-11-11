use add_one;
use add_one::rand;

fn main() {
    let num = 10;
    let new_num = add_one::add_one(num) + rand::random::<i32>();
    println!("New num: {new_num}");
}
