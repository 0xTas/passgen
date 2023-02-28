use clap::Parser;
use passgen::{
    Args,
    PassGen,
    MIN_LEN,
    MAX_LEN,
};

fn main() {
    let args = Args::parse();
    if args.length < MIN_LEN {
        eprintln!("Passwords must be between {MIN_LEN} and {MAX_LEN} in length.");
        return;
    };
    let generator = PassGen::new(args.length, args.spaces);
    
    println!("{}", generator.generate());
}
