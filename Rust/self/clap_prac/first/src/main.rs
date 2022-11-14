use first::{Args, check_accepted};
use clap::Parser;

fn main() {
    let args = Args::parse();
    if check_accepted(args) {
        println!("Success! The code was valid");
    }
    else {
        println!("Invalid code");
    }
}
