// Importing the Token enum from the token module
use crate::token::Token;

// Define the function that takes a vector of tokens and returns either a vector of strings
// or an error message (as a String) using  Result type
pub fn parse_expression(tokens: Vec<Token>) -> Result<Vec<String>, String> {
    // If the input token list is empty, return an error
    if tokens.is_empty() {
        return Err("Expression is empty.".to_string());
    }

    // Converting each token into its debug string format using map and collect into a new vector
    let result: Vec<String> = tokens.into_iter()
        .map(|token| format!("{:?}", token))  // Format each token as a string
        .collect();  // Collecting all formatted tokens into a vector

    // Returning the resulting vector wrapped in Ok to indicate success
    Ok(result)
}
