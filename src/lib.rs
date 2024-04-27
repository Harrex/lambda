use core::panic;

#[derive(Debug, Clone)]
pub enum LambdaToken {
    Var(char),
    Lambda(char, Box<LambdaToken>),
    App(Box<LambdaToken>, Box<LambdaToken>),
}

#[derive(Debug, Clone)]
pub enum NotQuiteLambdaToken {
    Var(char),
    Lambda(char, Vec<NotQuiteLambdaToken>),
    App,
}

#[derive(Debug)]
pub enum LambdaNode {
    Lambda,
    Dot,
    Var(char),
    LParen,
    RParen,
    App,
    True,
    False,
    And,
    Or,
}

pub fn lex(string_to_parse: String) -> Vec<LambdaNode> {
    let mut to_return = Vec::new();

    for character in string_to_parse.chars() {
        match character {
            '/' => to_return.push(LambdaNode::Lambda),
            'a'..='z' => to_return.push(LambdaNode::Var(character)),
            '.' => to_return.push(LambdaNode::Dot),
            ' ' => to_return.push(LambdaNode::App),
            '(' => to_return.push(LambdaNode::LParen),
            ')' => to_return.push(LambdaNode::RParen),
            'T' => to_return.push(LambdaNode::True),
            'F' => to_return.push(LambdaNode::False),
            '&' => to_return.push(LambdaNode::And),
            '|' => to_return.push(LambdaNode::Or),
            _ => eprintln!("Unrecognized character '{character}'"),
        }
    }

    to_return
}

struct NodeCounter<T> {
    node_list: Vec<T>,
    index: usize,
}

impl<T> NodeCounter<T> {
    fn new(node_list: Vec<T>) -> NodeCounter<T> {
        NodeCounter {
            node_list,
            index: 0,
        }
    }
    fn next(&mut self) -> &T {
        let to_return = &self.node_list[self.index];
        self.index += 1;
        to_return
    }
    fn has_next(&mut self) -> bool {
        self.node_list.len() > self.index
    }
}

/// Helper: Parse to the next (, ), /, or other stopper (I can't think of any at the moment).
/// This will return the same kind of AST as parse_lexed_to_ast() does, so they will be merged
fn parse_body_helper(node_counter: &mut NodeCounter<LambdaNode>) -> Vec<NotQuiteLambdaToken> {
    let mut to_be_half_finished_return: Vec<NotQuiteLambdaToken> = Vec::new();
    let mut first_round: bool = true;
    let mut waiting_for_bracket: bool = false;

    while node_counter.has_next() {
        let n = node_counter.next();
        match n {
            LambdaNode::Lambda => to_be_half_finished_return.push(NotQuiteLambdaToken::Lambda(
                match node_counter.next() {
                    LambdaNode::Var(a) => *a,
                    _ => panic!("Must have var after this"),
                },
                match node_counter.next() {
                    LambdaNode::Dot => parse_body_helper(node_counter), //This is already a
                    //mutable borrow
                    _ => panic!("Lambda must have a dot: /x.()"),
                },
            )),

            LambdaNode::Dot => panic!("Too many dots!"),

            LambdaNode::Var(v) => to_be_half_finished_return.push(NotQuiteLambdaToken::Var(*v)),

            LambdaNode::LParen => {
                if first_round {
                    waiting_for_bracket = true
                } else {
                    panic!("Too many left brackets! Did you miss a space?");
                }
            }

            LambdaNode::RParen => {
                if waiting_for_bracket {
                    return to_be_half_finished_return;
                } else {
                    panic!("Too many right brackets!")
                }
            }

            LambdaNode::App => {
                to_be_half_finished_return.insert(0, NotQuiteLambdaToken::App);
            }

            LambdaNode::True => {
                let mut node_counter = NodeCounter::new(lex(String::from("/p.(/q.(p))")));
                to_be_half_finished_return.push(parse_body_helper(&mut node_counter)[0].clone());
            }
            LambdaNode::False => {
                let mut node_counter = NodeCounter::new(lex(String::from("/p.(/q.(q))")));
                to_be_half_finished_return.push(parse_body_helper(&mut node_counter)[0].clone());
            },
            LambdaNode::And   => {
                let mut node_counter = NodeCounter::new(lex(String::from("/p.(/q.(q p q))")));
                to_be_half_finished_return.push(parse_body_helper(&mut node_counter)[0].clone());
            },
            LambdaNode::Or    => {
                let mut node_counter = NodeCounter::new(lex(String::from("/p.(/q.(p p q))")));
                to_be_half_finished_return.push(parse_body_helper(&mut node_counter)[0].clone());
            },
        }                        
        first_round = false;
    }

    to_be_half_finished_return
}

fn not_quite_to_lambda_token(not_quite: NotQuiteLambdaToken) -> Box<LambdaToken> {
    return match not_quite {
        NotQuiteLambdaToken::Var(v) => Box::new(LambdaToken::Var(v)),
        NotQuiteLambdaToken::Lambda(head, body) => {
            Box::new(LambdaToken::Lambda(head, finish_the_job(body)))
        }
        NotQuiteLambdaToken::App => panic!("Shouldn't be here"),
    };
}

fn resolve_application_nonsense(
    node_counter: &mut NodeCounter<NotQuiteLambdaToken>,
) -> Box<LambdaToken> {
    let n = node_counter.next();
    match n {
        NotQuiteLambdaToken::App => Box::new(LambdaToken::App(
            resolve_application_nonsense(node_counter),
            not_quite_to_lambda_token(node_counter.next().clone()),
        )),
        _ => Box::new(LambdaToken::App(
            not_quite_to_lambda_token(n.clone()),
            not_quite_to_lambda_token(node_counter.next().clone()),
        )),
    }
}

fn finish_the_job(half_finished_ast: Vec<NotQuiteLambdaToken>) -> Box<LambdaToken> {
    let mut node_counter = NodeCounter::new(half_finished_ast);
    let to_return = match node_counter.next() {
        NotQuiteLambdaToken::Var(v) => Box::new(LambdaToken::Var(*v)),
        NotQuiteLambdaToken::Lambda(head, tail) => {
            Box::new(LambdaToken::Lambda(*head, finish_the_job(tail.clone())))
        }
        NotQuiteLambdaToken::App => resolve_application_nonsense(&mut node_counter),
    };
    to_return
}

/// Parses the following symbols:
/// [a-z] -> Var
/// / -> Lambda
/// (...) [a-z] -> App
pub fn parse_lexed_to_ast(lexed_string_to_parse: Vec<LambdaNode>) -> Box<LambdaToken> {
    let halfway: Vec<NotQuiteLambdaToken>;
    let all_the_way: Box<LambdaToken>;
    let mut node_counter = NodeCounter::new(lexed_string_to_parse);
    halfway = parse_body_helper(&mut node_counter);
    dbg!(&halfway);
    all_the_way = finish_the_job(halfway);
    all_the_way
}

// --------------------------

// Beta Reduction time!!!

pub fn beta_reduce(calc: Box<LambdaToken>) -> Box<LambdaToken> {
    dbg!(&calc);
    match *(calc.clone()) {
        LambdaToken::App(a, b) => match *(a.clone()) {
            LambdaToken::App(_, _) => beta_reduce(Box::new(LambdaToken::App(beta_reduce(a), b))),
            LambdaToken::Lambda(head, body) => beta_reduce(substitute(body, head, &b)),
            LambdaToken::Var(_) => calc,
        },
        _ => calc,
    }
}

fn substitute(
    thing_to_substitute: Box<LambdaToken>,
    from: char,
    to: &Box<LambdaToken>,
) -> Box<LambdaToken> {
    match *(thing_to_substitute.clone()) {
        LambdaToken::Var(v) => {
            if v == from {
                to.clone()
            } else {
                thing_to_substitute
            }
        }
        LambdaToken::Lambda(head, body) => {
            if head == from {
                thing_to_substitute // If head == from, then the variable has been shadowed in
                                    // this function, and we should leave it alone.
            } else {
                Box::new(LambdaToken::Lambda(head, substitute(body, from, to)))
            }
        }
        LambdaToken::App(a, b) => Box::new(LambdaToken::App(
            substitute(a, from, to),
            substitute(b, from, to),
        )),
    }
}
