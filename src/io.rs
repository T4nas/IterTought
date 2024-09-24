use std::io::{self, Write};

pub fn get_user_query() -> String {
    print!("Enter your query: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read line");

    query.trim().to_string()
}
