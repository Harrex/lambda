use std::env;

use lambda;

fn main() {
    let args: Vec<String> = env::args().collect();
    let string = args[1].clone();
    let lexed_string = lambda::lex(string);
    let parsed_string = lambda::parse_lexed_to_ast(lexed_string);
    let reduced_string = lambda::beta_reduce(parsed_string);
    println!("{:?}", reduced_string);
}
