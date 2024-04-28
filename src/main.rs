use std::env;

use lambda;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        _ => {
            let parsed_string = lambda::parse_string(args[1].clone());
            let reduced_string = lambda::beta_reduce(parsed_string);
            println!("{:?}", reduced_string);
        }
    }
}
