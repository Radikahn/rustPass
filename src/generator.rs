use rand::Rng;
use rand::distr::Alphabetic;
use std::ops::Range;

const MASTER_CHAR: [char; 16] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '[', ']', ',', '.',
];

fn generate_numbers(size: Range<usize>) -> i32 {
    let mut rng = rand::rng();
    return rng.random_range::<usize, _>(size) as i32;
}

fn symbol_salt() -> char {
    return MASTER_CHAR[generate_numbers(1_usize..MASTER_CHAR.len()) as usize];
}

fn letter_salt() -> String {
    let mut rng = rand::rng();
    return (0..2).map(|_| rng.sample(Alphabetic) as char).collect();
}

pub fn password(length: i8) -> String {
    let mut gen_count: i8 = 0;
    let mut password = String::from("");

    loop {
        if gen_count > length {
            break;
        };

        let which: i32 = generate_numbers(1..4);

        match which {
            1 => password.push_str(generate_numbers(0..9).to_string().trim()),
            2 => password.push_str(letter_salt().to_string().trim()),
            3 => password.push_str(symbol_salt().to_string().trim()),
            _ => (),
        }

        gen_count += 1;
    }

    return password[0..(length as usize)].to_string();
}
