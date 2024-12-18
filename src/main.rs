mod methods;
use methods::select::parse_select;


fn main() {
    let query = "SELECT col1, col2 FROM my_table";
    match parse_select(&query) {
        Ok((remaining, (table,columns))) => {
                println!("Table: {}", table);
                println!("Columns: {:?}", columns);
                println!("Remaining: {}", remaining);
        }
        Err(err) => {
            eprintln!("Failed to parse query: {:?}", err);
        }
    }
}