mod methods;
use methods::{insert::parse_insert, select::parse_select};

fn main() {
    let query = "SELECT col1, col2 FROM my_table";
    let query2 = "INSERT INTO users (user_id, user_name, user_age) VALUES (1, 'Alice', 30);";
    match parse_select(&query) {
        Ok((remaining, (table, columns))) => {
            println!("Table: {}", table);
            println!("Columns: {:?}", columns);
            println!("Remaining: {}", remaining);
        }
        Err(err) => {
            eprintln!("Failed to parse query: {:?}", err);
        }
    }
    println!("-----------------------------");
    match parse_insert(&query2) {
        Ok((remaining, (table, columns, values))) => {
            println!("Table: {}", table);
            println!("Columns: {:?}", columns);
            println!("Values: {:?}", values);
            println!("Remaining: {}", remaining);
        }

        Err(err) => {
            eprintln!("Failed to parse query: {:?}", err);
        }
    }
}
