use std::{
    env,
    io::{self, stdout, Write},
};

use lambda;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "-i" => {
            repl();
        }
        _ => {
            let parsed_string = lambda::parse_string(args[1].clone());
            let reduced_string = lambda::beta_reduce(parsed_string);
            println!("{}", lambda::display_as_text(reduced_string));
        }
    }
}

fn repl() {
    println!(
        "Welcome to Harrex's lambda calculus REPL: ^D to exit. 
    - Use lowercase letters a~z for variables, and / or λ for λ.
    - Some conveniences: &, |, T, F"
    );
    let mut previous_inputs: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => eprintln!("Failed to read input"),
        }
        input = input.strip_suffix('\n').unwrap().to_string();
        previous_inputs.push(input.clone());
        let parsed_string = lambda::parse_string(input);
        let reduced_string = lambda::beta_reduce(parsed_string);
        println!("{}", lambda::display_as_text(reduced_string));
    }
}
