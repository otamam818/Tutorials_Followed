use clap::Parser;

/// A program to greet someone based on their codename
#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The name given to them
    pub code_name: String,

    /// The number given to them
    pub code_number: u32,
}

pub fn check_accepted(args: Args) -> bool {
    let accepted = [
        Args {
            code_name: String::from("Tahmin"),
            code_number: 00,
        },
        Args {
            code_name: String::from("Oisho"),
            code_number: 11,
        },
        Args {
            code_name: String::from("Tamam"),
            code_number: 6,
        }
    ];

    accepted.contains(&args)
}

