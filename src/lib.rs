use clap::Parser;
use rand::{
    thread_rng, 
    seq::SliceRandom,
    distributions::{
        Uniform,
        Distribution,
    }
};


pub const MIN_LEN: u8 = 8;
pub const MAX_LEN: u8 = u8::MAX;

const SPACE_CHAR: char = ' ';
const DIGITS: &[u8] = &[0,1,2,3,4,5,6,7,8,9];
const SYMBOLS: &[char] = &['`', '~', '!', '@', '#', '$', '%', '^', 
                            '&', '*', '(', ')', '-', '_', '=', '+',
                            '[', ']', '{', '}', '\\', '|', ';', ':',
                            '\'', '"', ',', '.', '<', '>', '/', '?'];
const LETTERS_LOWER: &[char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
                                'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
                                'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const LETTERS_UPPER: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
                                'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
                                'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];


#[derive(Debug, Parser)]
#[command(author, version)]
pub struct Args {
    /// The desired length of your generated password
    #[arg(short, long, default_value_t = 20)]
    pub length: u8,

    /// Include space characters in the generated password
    #[arg(short, long, default_value_t = false)]
    pub spaces: bool
}


#[derive(Debug)]
pub struct PassGen {
    length: u8,
    use_spaces: bool
}

impl PassGen {
    pub fn new(length: u8, use_spaces: bool) -> Self {
        PassGen {
            length,
            use_spaces
        }
    }

    pub fn generate(&self) -> String {
        let mut rng = thread_rng();
        let mut key = String::with_capacity(self.length as usize);

        if self.use_spaces {
            let set_choice: Uniform<u8> = Uniform::new(0, 100);
            
            for _ in 0..self.length {
                let set_choice: u8 = set_choice.sample(&mut rng);

                if set_choice <= 7 {
                    key.push(SPACE_CHAR);
                }else if set_choice < 35 {
                    key.push(
                        char::from_digit(
                            *DIGITS.choose(&mut rng).expect("Slice should not be empty!") as u32,
                            10).expect("Expected a valid digit!")
                    );
                }else if set_choice < 55 {
                    key.push(*SYMBOLS.choose(&mut rng).expect("Slice should not be empty!"));
                }else if set_choice < 75 {
                    key.push(*LETTERS_LOWER.choose(&mut rng).expect("Slice should not be empty!"));
                }else {
                    key.push(*LETTERS_UPPER.choose(&mut rng).expect("Slice should not be empty!"));
                };
            };
        }else {
            let set_choice: Uniform<u8> = Uniform::new(0, 80);

            for _ in 0..self.length {
                let set_choice: u8 = set_choice.sample(&mut rng);
                if set_choice < 20 {
                    key.push(
                        char::from_digit(
                            *DIGITS.choose(&mut rng).expect("Slice should not be empty!") as u32,
                            10).expect("Expected a valid digit!")
                    );
                }else if set_choice < 40 {
                    key.push(*SYMBOLS.choose(&mut rng).expect("Slice should not be empty!"));
                }else if set_choice < 60 {
                    key.push(*LETTERS_LOWER.choose(&mut rng).expect("Slice should not be empty!"));
                }else {
                    key.push(*LETTERS_UPPER.choose(&mut rng).expect("Slice should not be empty!"));
                };
            };
        };

        key
    }
}