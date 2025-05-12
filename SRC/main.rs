mod token;         // Declare the token module
mod tokenizer;     // Declare the tokenizer module
mod pratt_parser;  // Declare the pratt_parser module
mod sql_parser;    // Declare the sql_parser module

use std::io::{self, Write}; // Import input/output and Write trait for flushing
use tokenizer::tokenize;    // Import tokenize function
use pratt_parser::parse_expression; // Import expression parser
use sql_parser::parse_select;       // Import SQL SELECT parser

fn main() {
    println!("Welcome to the SQL Parser CLI!"); // Initial welcome message

    loop {
        print!("Statement: "); // Prompt for input
        io::stdout().flush().unwrap();  // Force print to show immediately

        let mut input = String::new(); // Create mutable input buffer
        io::stdin().read_line(&mut input).unwrap(); // Read user input
        let statement = input.trim(); // Trim whitespace and newlines

        let tokens = tokenize(statement); // Tokenize the input string
        println!("Tokens: {:?}", tokens); // Show tokens

        // Try to parse as an arithmetic expression
        match parse_expression(tokens.clone()) {
            Ok(result) => println!("Parsed Expression: {:?}", result),
            Err(err) => println!("Error: {}", err),
        }

        // Try to parse as a SQL SELECT statement
        match parse_select(tokens) {
            Ok(select_result) => println!("{}", select_result),
            Err(err) => println!("Error: {}", err),
        }
    }
}
