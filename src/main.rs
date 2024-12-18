mod methods;
use methods::{insert::parse_insert, select::parse_select};


fn main() {
    let query = "SELECT col1, col2 FROM my_table";
    let query2 = "INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country)
VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway');";
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
    println!("-----------------------------");
    match parse_insert(&query2) {
        Ok((_remaining, (table, _columns))) => {
            println!("Table: {}", table);
        }

        Err(err) => {
            eprintln!("Failed to parse query: {:?}", err);
        }
    }
}