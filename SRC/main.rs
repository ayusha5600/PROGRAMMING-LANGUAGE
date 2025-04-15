mod parser;
fn main() {
    let sql = "SELECT name, age FROM users WHERE age = 18";

    match parser::parse_select(sql) {
        Ok((remaining_input, _)) => {
            println!("Matched keyword: 'SELECT'");
            let remaining_input = remaining_input.trim();

            match parser::parse_columns(remaining_input) {
                Ok((remaining_input, columns)) => {
                    println!("Columns: {:?}", columns);
                    let remaining_input = remaining_input.trim();

                    match parser::parse_from(remaining_input) {
                        Ok((remaining_input, table_name)) => {
                            println!("Table: {}", table_name);
                            let remaining_input = remaining_input.trim();

                            // Add this part to parse the WHERE clause
                            match parser::parse_where(remaining_input) {
                                Ok((remaining_input, (left, op, right))) => {
                                    println!("Condition: {} {} {}", left, op, right);
                                    println!("Remaining input: {}", remaining_input);
                                }
                                Err(_) => {
                                    println!("No WHERE clause or failed to parse condition.");
                                }
                            }
                        }
                        Err(e) => println!("Error parsing FROM clause: {:?}", e),
                    }
                }
                Err(e) => println!("Error parsing columns: {:?}", e),
            }
        }
        Err(e) => println!("Error parsing SELECT: {:?}", e),
    }
}
