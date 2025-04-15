# SQL Parser in Rust
A Rust-based parser for basic SQL queries (SELECT, FROM, WHERE).
# Description
 SQL parser implemented in Rust can parse SQL queries with the `SELECT`, `FROM`, and `WHERE` clauses, extracting column names, table names, and conditions. The parser currently supports simple equality (`=`) conditions in the `WHERE` clause.
# Installation
 Clone the repository:
   git clone https://github.com/ayusha5600/sql-parser.git
   cd sql-parser
#  Install dependencies:
   cargo build
#  To run the parser:
   cargo run
# To test the parser:
   cargo test
# Build the project
cargo build
# Run the parser
cargo run
#  Example SQL input:
Editing the main.rs file to use my own SQL string:
let sql = "SELECT name, age FROM users WHERE age = 18";
Output:
Matched keyword: 'SELECT'
Columns: ["name", "age"]
Table: users
Condition: age = 18
Remaining input:
# To run any available tests:
cargo test
