# Harrex's Lambda Calculus Program
A little program written in Rust to evaluate lambda calculus expressions.

## Usage
Command line:
```
lambda "λx.x a"
```
REPL
```
lambda -i
```

### Syntax
For variables, use a lowercase letter
For lambdas, use either `λ` or `/`
Otherwise, it's the same as regular lambda calculus

### Conveniences
There are a couple of common functions that I've included:
|   |   |   |
|---|---|---|
| `T` | True  | `λp.λq.(p)`|
| `F` | False | `λp.λq.(q)`|
| `&` | And   | `λp.λq.(q p q)`|
| `|` | Or    | `λp.λq.(p p q)`|

## Installation
### Linux (Binary)
For a binary, see releases
To compile, see Other

### Other (Compile it yourself)
Clone this repository, and run 
```cargo build --release```
You'll find a binary in `./target/release`


