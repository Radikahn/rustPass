use std::env;
mod generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut length: i8 = 0;

    if args.len() > 1 {
        length = args[1]
            .parse()
            .expect("Expecting Password Length Argument MAX VALUE -> 126");
    }

    let local: &str = &generator::password(length);

    println!("Generated Pass: {}", local);
    println!("pass length: {}", local.len());
}
